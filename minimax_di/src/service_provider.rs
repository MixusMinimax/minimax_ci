use std::any::Any;
use std::error::Error;

use errors::ServiceNotFoundError;
use service_traits::{
    GenericServiceProvider, ServiceCollection, ServiceKey, ServiceProvider, ServiceProviderBuilder,
};

pub struct ServiceProviderImpl {
    services: Box<dyn ServiceCollection>,
}

impl ServiceProviderImpl {
    pub fn new(services: Box<dyn ServiceCollection>) -> Self {
        ServiceProviderImpl { services }
    }
}

impl ServiceProvider for ServiceProviderImpl {
    fn get_service_any(&self, key: &ServiceKey) -> Result<Box<dyn Any>, Box<dyn Error>> {
        let descriptors = self.services.get_services(key);
        let descriptor = *descriptors
            .first()
            .ok_or_else(|| Box::new(ServiceNotFoundError(key.clone())))?;
        descriptor.new_service(self)
    }
}

impl ServiceProviderBuilder for Box<dyn ServiceCollection> {
    fn build(self) -> Box<dyn ServiceProvider> {
        // TODO: Check for circular dependencies
        Box::new(ServiceProviderImpl::new(self))
    }
}

impl GenericServiceProvider for Box<dyn ServiceProvider> {
    fn get_service<S: ?Sized + 'static>(&self, key: &ServiceKey) -> Result<Box<S>, Box<dyn Error>> {
        Ok(*self
            .as_ref()
            .get_service_any(key)?
            .downcast::<Box<S>>()
            .map_err(|_| Box::new(ServiceNotFoundError(key.clone())))?)
    }
}
