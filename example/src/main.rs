use std::any::Any;
use std::error::Error;

use lazy_static::lazy_static;

use minimax_di::service_traits::ServiceLifetime::Transient;
use minimax_di::service_traits::{
    GenericServiceProvider, Service, ServiceDescriptor, ServiceKey, ServiceLifetime,
    ServiceProvider, ServiceProviderBuilder,
};

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

    fn new_service(
        &self,
        _service_provider: &dyn ServiceProvider,
    ) -> Result<Box<dyn Any>, Box<dyn Error>> {
        Ok(Box::new(
            ExampleServiceImpl::new(())? as Box<dyn ExampleService>
        ))
    }
}

impl Service<(), dyn ExampleService> for ExampleServiceImpl {
    fn new(_deps: ()) -> Result<Box<Self>, Box<dyn Error>> {
        Ok(Box::new(ExampleServiceImpl {}))
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
