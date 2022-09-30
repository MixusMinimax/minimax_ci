#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct ServiceKey(String);

pub trait ServiceCollection {
    fn register_service<S>(&mut self, service_descriptor: S) -> &mut Self
    where
        S: ServiceDescriptor + 'static;
}

pub trait ServiceProvider {}

pub trait Service {}

pub enum ServiceLifetime {
    Singleton,
    Scoped,
    Transient,
}

/// For every service, a service descriptor is created that implements this trait.
pub trait ServiceDescriptor: Service {
    /// Gets the lifetime of this service.
    fn lifetime(&self) -> ServiceLifetime;

    /// Gets the key of this service.
    fn identifier(&self) -> ServiceKey;

    /// Gets the dependency keys of this service.
    fn dependencies(&self) -> Vec<ServiceKey>;

    /// Constructs a new instance of the service based on the dependencies.
    fn new_service(&self, service_provider: &dyn ServiceProvider) -> Box<dyn Service>;
}
