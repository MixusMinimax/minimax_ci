use std::any::{Any, TypeId};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::sync::Arc;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct ServiceKey(pub String);

pub trait AnyArc: Any {
    fn clone_arc(&self) -> Box<dyn AnyArc>;
}

impl dyn AnyArc {
    pub fn is<T: Any>(&self) -> bool {
        // Get `TypeId` of the type this function is instantiated with.
        let t = TypeId::of::<T>();

        // Get `TypeId` of the type in the trait object (`self`).
        let concrete = self.type_id();

        // Compare both `TypeId`s on equality.
        t == concrete
    }
}

impl<T: ?Sized + 'static> AnyArc for Arc<T> {
    fn clone_arc(&self) -> Box<dyn AnyArc> {
        Box::new(self.clone())
    }
}

pub trait Service<Deps, Interface: ?Sized>: Sized {
    fn new(deps: Deps) -> Result<Self, Box<dyn Error>>;
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
    ) -> Result<Box<dyn AnyArc>, Box<dyn Error>>;
}

pub trait GenericServiceProvider {
    fn get_service<S: ?Sized + Sync + Send + 'static>(
        &self,
        key: &ServiceKey,
    ) -> Result<Arc<S>, Box<dyn Error>>;
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
    ) -> Result<Box<dyn AnyArc>, Box<dyn Error>>;
}

// Display

impl Display for ServiceKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.as_str())?;
        Ok(())
    }
}
