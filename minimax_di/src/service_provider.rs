use std::any::Any;

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
    fn get_service_any(&self, key: &ServiceKey) -> Option<Box<dyn Any>> {
        let descriptors = self.services.get_services(key);
        let descriptor = *descriptors.first()?;
        Some(descriptor.new_service(self))
    }
}

impl ServiceProviderBuilder for Box<dyn ServiceCollection> {
    fn build(self) -> Box<dyn ServiceProvider> {
        Box::new(ServiceProviderImpl::new(self))
    }
}

impl GenericServiceProvider for Box<dyn ServiceProvider> {
    fn get_service<S: ?Sized + 'static>(&self, key: &ServiceKey) -> Option<Box<S>> {
        Some(*self.as_ref().get_service_any(key)?.downcast::<Box<S>>().ok()?)
    }
}
