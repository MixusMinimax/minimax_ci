use minimax_proc::minimax_service;

#[test]
fn test_service_def() {
    minimax_service! {
        type interface = dyn ExampleService;
        type descriptor = ExampleServiceDescriptor;
        let lifetime = Singleton;

        fn new((): ()) -> Result<Box<ExampleServiceImpl>, Box<dyn Error>> {
            println!("ExampleServiceImpl::new(())");
            Ok(Box::new(ExampleServiceImpl {}))
        }
    }
}
