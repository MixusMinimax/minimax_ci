#[macro_export]
macro_rules! minimax_service {
    {
        type interface = $interface:ty;
        type descriptor = $descriptor:ident;
        let lifetime = $lifetime:expr;

        fn new(($($args:ident), *): $($deps:ty), +) -> Result<Box<$imp:ty>, Box<dyn Error>> $bl:block
    } => {
        impl Service<$($deps), *, $interface> for $imp {
            fn new(($($args:ident), *) : $($deps), +) -> Result<Box<$imp>, Box<dyn Error>> $bl
        }

        struct $descriptor;

        impl ServiceDescriptor for $descriptor {
            fn lifetime(&self) -> ServiceLifetime {
                $lifetime
            }

            fn identifier(&self) -> ServiceKey {
                ServiceKey(stringify_service_ref!($interface).to_string())
            }

            fn dependencies(&self) -> Vec<ServiceKey> {
                vec![]
            }

            fn service_type(&self) -> TypeId {
                TypeId::of::<$imp>()
            }

            fn new_service(
                &self,
                _service_provider: &dyn ServiceProvider,
            ) -> Result<Arc<dyn Any + Send + Sync>, Box<dyn Error>> {
                Ok(Arc::new(
                    <$imp>::new(())? as Box<add_traits![$interface, core::marker::Send, core::marker::Sync, 'static]>
                ))
            }
        }
    };
}
