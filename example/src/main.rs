use std::any::{Any, TypeId};
use std::error::Error;
use std::sync::Arc;

use lazy_static::lazy_static;

use minimax_di::service_traits::{
    GenericServiceProvider, Service, ServiceDescriptor, ServiceKey, ServiceLifetime,
    ServiceProvider, ServiceProviderBuilder,
};
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

    fn service_type(&self) -> TypeId {
        TypeId::of::<ExampleServiceImpl>()
    }

    fn new_service(
        &self,
        _service_provider: &dyn ServiceProvider,
    ) -> Result<Arc<dyn Any + Send + Sync>, Box<dyn Error>> {
        Ok(Arc::new(ExampleServiceImpl::new(())? as Box<dyn ExampleService + Send + Sync>))
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

    let service_provider = services.build().unwrap();

    let example_service: Arc<Box<dyn ExampleService + Send + Sync>> = service_provider
        .get_service::<dyn ExampleService + Send + Sync>(&EXAMPLE_IDENTIFIER)
        .unwrap();
    example_service.say_hello();
}
