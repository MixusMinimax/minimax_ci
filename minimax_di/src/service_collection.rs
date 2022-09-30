use std::collections::HashMap;

use service_traits::{ServiceCollection, ServiceDescriptor, ServiceKey};

pub struct ServiceCollectionImpl {
    service_descriptors: HashMap<ServiceKey, Vec<Box<dyn ServiceDescriptor>>>,
}

impl ServiceCollection for ServiceCollectionImpl {
    fn register_service<S>(&mut self, service_descriptor: S) -> &mut Self
    where
        S: ServiceDescriptor + 'static,
    {
        let descriptors = self
            .service_descriptors
            .entry(service_descriptor.identifier())
            .or_insert_with(Vec::new);

        descriptors.push(Box::new(service_descriptor));

        self
    }
}
