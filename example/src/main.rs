use std::any::{Any, TypeId};
use std::error::Error;
use std::sync::Arc;

use lazy_static::lazy_static;

use minimax_di::minimax_service;
use minimax_di::service_traits::{
    GenericServiceProvider, Service, ServiceDescriptor, ServiceKey, ServiceLifetime,
    ServiceProvider, ServiceProviderBuilder,
};
use minimax_di::service_traits::ServiceLifetime::Singleton;
use minimax_proc::{add_traits, stringify_service_ref};

pub trait ExampleService {
    fn say_hello(&self);
}

lazy_static! {
    pub static ref EXAMPLE_IDENTIFIER: ServiceKey = ServiceKey(String::from("dyn ExampleService"));
}

pub struct ExampleServiceImpl {}

impl ExampleService for ExampleServiceImpl {
    fn say_hello(&self) {
        println!("Hello, World!");
    }
}

minimax_service! {
    type interface = dyn ExampleService;
    type descriptor = ExampleServiceDescriptor;
    let lifetime = Singleton;

    fn new((): ()) -> Result<Box<ExampleServiceImpl>, Box<dyn Error>> {
        println!("ExampleServiceImpl::new(())");
        Ok(Box::new(ExampleServiceImpl {}))
    }
}

fn main() {
    let mut services = minimax_di::new_service_collection();

    println!("{}", ExampleServiceDescriptor.identifier());

    services.register_service(Box::new(ExampleServiceDescriptor));

    let service_provider = services.build().unwrap();

    let example_service: Arc<Box<dyn ExampleService + Send + Sync>> = service_provider
        .get_service::<dyn ExampleService + Send + Sync>(&EXAMPLE_IDENTIFIER)
        .unwrap();
    example_service.say_hello();

    // If singleton, then the constructor will not be called again here:

    let example_service: Arc<Box<dyn ExampleService + Send + Sync>> = service_provider
        .get_service::<dyn ExampleService + Send + Sync>(&EXAMPLE_IDENTIFIER)
        .unwrap();
    example_service.say_hello();
}
