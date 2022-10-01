use std::any::Any;

use service_traits::{ServiceCollection, ServiceKey, ServiceProvider};

pub struct ServiceProviderImpl {
    services: Box<dyn ServiceCollection>
}

impl ServiceProvider for ServiceProviderImpl {
    fn get_service(&self, key: &ServiceKey) -> Option<Box<dyn Any>> {
        let descriptor = 0;
        None
    }
}