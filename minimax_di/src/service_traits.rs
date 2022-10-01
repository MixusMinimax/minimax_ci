use std::any::Any;
use std::error::Error;
use std::sync::Arc;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct ServiceKey(pub String);

pub(crate) trait Service<Deps> {
    fn new(deps: Deps) -> Result<Box<Self>, Box<dyn Error>>;
}

pub trait ServiceCollection {
    fn register_service(
        &mut self,
        service_descriptor: Box<dyn ServiceDescriptor>,
    ) -> &mut dyn ServiceCollection;

    fn get_services(&self, service_key: &ServiceKey) -> Vec<&Box<dyn ServiceDescriptor>>;
}

pub trait ServiceProvider {
    fn get_service(&self, key: &ServiceKey) -> Option<Box<dyn Any>>;
}

pub enum ServiceLifetime {
    Singleton,
    Scoped,
    Transient,
}

/// For every service, a service descriptor is created that implements this trait.
pub trait ServiceDescriptor {
    /// Gets the lifetime of this service.
    fn lifetime(&self) -> ServiceLifetime;

    /// Gets the key of this service.
    fn identifier(&self) -> ServiceKey;

    /// Gets the dependency keys of this service.
    fn dependencies(&self) -> Vec<ServiceKey>;

    /// Constructs a new instance of the service based on the dependencies.
    fn new_service(&self, service_provider: &dyn ServiceProvider) -> Box<dyn Any>;
}
