use std::collections::HashMap;

use service_traits::{ServiceCollection, ServiceDescriptor, ServiceKey};

pub(crate) struct ServiceCollectionImpl {
    service_descriptors: HashMap<ServiceKey, Vec<Box<dyn ServiceDescriptor>>>,
}

impl ServiceCollectionImpl {
    pub fn new() -> Self {
        ServiceCollectionImpl {
            service_descriptors: HashMap::new(),
        }
    }
}

impl ServiceCollection for ServiceCollectionImpl {
    fn register_service(
        &mut self,
        service_descriptor: Box<dyn ServiceDescriptor>,
    ) -> &mut dyn ServiceCollection {
        let descriptors = self
            .service_descriptors
            .entry(service_descriptor.identifier())
            .or_insert_with(Vec::new);

        descriptors.push(service_descriptor);

        self
    }

    fn get_services(&self, service_key: &ServiceKey) -> Vec<&dyn ServiceDescriptor> {
        match self.service_descriptors.get(service_key) {
            None => Vec::new(),
            Some(descriptors) => descriptors.iter().map(Box::as_ref).collect(),
        }
    }
}
