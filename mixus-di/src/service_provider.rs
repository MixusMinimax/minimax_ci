use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, RwLock};

use crate::errors::ServiceNotFoundError;
use crate::service_traits::ServiceLifetime::Singleton;
use crate::service_traits::{
    AnyArc, GenericServiceProvider, ServiceCollection, ServiceKey, ServiceProvider,
    ServiceProviderBuilder,
};

pub struct ServiceProviderImpl {
    services: Box<dyn ServiceCollection>,
    singletons: Arc<RwLock<HashMap<TypeId, Box<dyn AnyArc>>>>,
}

impl ServiceProviderImpl {
    pub fn new(services: Box<dyn ServiceCollection>) -> Self {
        ServiceProviderImpl {
            services,
            singletons: RwLock::new(HashMap::new()).into(),
        }
    }
}

impl ServiceProvider for ServiceProviderImpl {
    fn get_service_any(&self, key: &ServiceKey) -> Result<Box<dyn AnyArc>, Box<dyn Error>> {
        let descriptors = self.services.get_services(key);
        let descriptor = *descriptors
            .first()
            .ok_or_else(|| Box::new(ServiceNotFoundError(key.clone())))?;
        let type_id = descriptor.service_type();
        if let Singleton = descriptor.lifetime() {
            let read = self.singletons.read().unwrap();
            if let Some(existing) = read.get(&type_id) {
                return Ok(existing.clone_arc());
            }
        }
        let service = descriptor.new_service(self)?;
        if let Singleton = descriptor.lifetime() {
            let mut write = self.singletons.write().unwrap();
            write.insert(type_id, service.clone_arc());
        }
        Ok(service)
    }
}

impl ServiceProviderBuilder for Box<dyn ServiceCollection> {
    fn build(self) -> Result<Box<dyn ServiceProvider>, Box<dyn Error>> {
        // TODO: Check for circular dependencies
        Ok(Box::new(ServiceProviderImpl::new(self)))
    }
}

impl GenericServiceProvider for Box<dyn ServiceProvider> {
    fn get_service<S: ?Sized + Sync + Send + 'static>(
        &self,
        key: &ServiceKey,
    ) -> Result<Arc<S>, Box<dyn Error>> {
        fn downcast<T: Any>(this: Box<dyn AnyArc>) -> Result<Box<T>, Box<dyn AnyArc>> {
            if this.is::<T>() {
                Ok(downcast_unchecked::<T>(this))
            } else {
                Err(this)
            }
        }

        fn downcast_unchecked<T: Any>(this: Box<dyn AnyArc>) -> Box<T> {
            unsafe {
                let (raw, alloc): (*mut dyn AnyArc, _) = Box::into_raw_with_allocator(this);
                Box::from_raw_in(raw as *mut T, alloc)
            }
        }

        Ok(*downcast::<Arc<S>>(self.get_service_any(key)?.clone_arc())
            .map_err(|_| Box::new(ServiceNotFoundError(key.clone())))?)
    }
}
