use std::any::Any;

use lazy_static::lazy_static;

use minimax_di::service_traits::{GenericServiceProvider, ServiceDescriptor, ServiceKey, ServiceLifetime, ServiceProvider, ServiceProviderBuilder};
use minimax_di::service_traits::ServiceLifetime::Transient;

pub trait ExampleService {
    fn say_hello(&self);
}

pub struct ExampleServiceDescriptor;

lazy_static! {
    pub static ref EXAMPLE_IDENTIFIER: ServiceKey = ServiceKey(String::from("ExampleService"));
}

pub struct ExampleServiceImpl {}

impl ExampleService for ExampleServiceImpl {
    fn say_hello(&self) {
        println!("Hello, World!");
    }
}

impl ServiceDescriptor for ExampleServiceDescriptor {
    fn lifetime(&self) -> ServiceLifetime {
        Transient
    }

    fn identifier(&self) -> ServiceKey {
        ServiceKey("ExampleService".to_string())
    }

    fn dependencies(&self) -> Vec<ServiceKey> {
        Vec::new()
    }

    fn new_service(&self, service_provider: &dyn ServiceProvider) -> Box<dyn Any> {
        Box::new(Box::new(ExampleServiceImpl {}) as Box<dyn ExampleService>)
    }
}

fn main() {
    let mut services = minimax_di::new_service_collection();

    services.register_service(Box::new(ExampleServiceDescriptor));

    let service_provider = services.build();

    let example_service: Box<dyn ExampleService> = service_provider
        .get_service::<dyn ExampleService>(&EXAMPLE_IDENTIFIER)
        .unwrap();

    example_service.say_hello();
}
