use std::any::Any;

use minimax_di::service_traits::{ServiceDescriptor, ServiceKey, ServiceLifetime, ServiceProvider};
use minimax_di::service_traits::ServiceLifetime::Transient;

pub trait ExampleService {
    fn say_hello();
}

pub struct ExampleServiceDescriptor;

pub struct ExampleServiceImpl {

}

impl ExampleService for ExampleServiceImpl {
    fn say_hello() {
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
        todo!()
    }
}

fn main() {
    println!("Hello, world!");

    let mut services = minimax_di::new_service_collection();

    services.register_service(Box::new(ExampleServiceDescriptor));
}
