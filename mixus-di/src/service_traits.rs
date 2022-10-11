use std::any::{Any, TypeId};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::sync::Arc;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct ServiceKey(pub String);

pub trait Service<Deps, Interface: ?Sized> {
    fn new(deps: Deps) -> Result<Box<Self>, Box<dyn Error>>;
}

pub trait ServiceCollection {
    fn register_service(
        &mut self,
        service_descriptor: Box<dyn ServiceDescriptor>,
    ) -> &mut dyn ServiceCollection;

    #[must_use]
    fn get_services(&self, service_key: &ServiceKey) -> Vec<&dyn ServiceDescriptor>;
}

pub trait ServiceProvider {
    fn get_service_any(
        &self,
        key: &ServiceKey,
    ) -> Result<Arc<dyn Any + Send + Sync>, Box<dyn Error>>;
}

pub trait GenericServiceProvider {
    fn get_service<S: ?Sized + Sync + Send + 'static>(
        &self,
        key: &ServiceKey,
    ) -> Result<Arc<Box<S>>, Box<dyn Error>>;
}

pub trait ServiceProviderBuilder {
    fn build(self) -> Result<Box<dyn ServiceProvider>, Box<dyn Error>>;
}

pub enum ServiceLifetime {
    Singleton,
    Scoped,
    Transient,
}

/// For every service, a service descriptor is created that implements this trait.
pub trait ServiceDescriptor {
    /// Gets the lifetime of this service.
    #[must_use]
    fn lifetime(&self) -> ServiceLifetime;

    /// Gets the key of this service.
    #[must_use]
    fn identifier(&self) -> ServiceKey;

    /// Gets the dependency keys of this service.
    #[must_use]
    fn dependencies(&self) -> Vec<ServiceKey>;

    /// Type of the service implementation
    #[must_use]
    fn service_type(&self) -> TypeId;

    /// Constructs a new instance of the service based on the dependencies.
    /// Because of the limitations of rust, this returns a Box of a Box to the actual service trait.
    fn new_service(
        &self,
        service_provider: &dyn ServiceProvider,
    ) -> Result<Arc<dyn Any + Send + Sync>, Box<dyn Error>>;
}

// Display

impl Display for ServiceKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.as_str())?;
        Ok(())
    }
}
