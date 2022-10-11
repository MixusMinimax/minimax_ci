use std::any::{Any, TypeId};
use std::error::Error;
use std::sync::Arc;

use lazy_static::lazy_static;

use minimax_di::service_traits::{
    GenericServiceProvider, Service, ServiceKey, ServiceLifetime,
    ServiceProvider, ServiceProviderBuilder,
};
use minimax_di::service_traits::ServiceLifetime::Singleton;

pub trait ExampleService {
    fn say_hello(&self);
}

lazy_static! {
    pub static ref EXAMPLE_IDENTIFIER: ServiceKey = ServiceKey(String::from("ExampleService"));
}

pub struct ExampleServiceImpl {}

impl ExampleService for ExampleServiceImpl {
    fn say_hello(&self) {
        println!("Hello, World!");
    }
}

// minimax_service! {
//     type interface = dyn ExampleService;
//     type descriptor = ExampleServiceDescriptor;
//     let lifetime = Singleton;
//
//     fn new((): ()) -> Result<Box<ExampleServiceImpl>, Box<dyn Error>> {
//         println!("ExampleServiceImpl::new(())");
//         Ok(Box::new(ExampleServiceImpl {}))
//     }
// }

// That macro will generate the following code:
// --start--

struct ExampleServiceDescriptor;

impl Service<(), dyn ExampleService> for ExampleServiceImpl {
    fn new((): ()) -> Result<Box<Self>, Box<dyn Error>> {
        println!("ExampleServiceImpl::new(())");
        Ok(Box::new(ExampleServiceImpl {}))
    }
}

impl minimax_di::service_traits::ServiceDescriptor for ExampleServiceDescriptor {
    fn lifetime(&self) -> ServiceLifetime {
        Singleton
    }

    fn identifier(&self) -> ServiceKey {
        EXAMPLE_IDENTIFIER.to_owned()
    }

    fn dependencies(&self) -> Vec<ServiceKey> {
        vec![]
    }

    fn service_type(&self) -> TypeId {
        TypeId::of::<ExampleServiceImpl>()
    }

    fn new_service(
        &self,
        _service_provider: &dyn ServiceProvider,
    ) -> Result<Arc<dyn Any + Send + Sync>, Box<dyn Error>> {
        Ok(Arc::new(
            ExampleServiceImpl::new(())? as Box<dyn ExampleService + Send + Sync>
        ))
    }
}

// --end--

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
