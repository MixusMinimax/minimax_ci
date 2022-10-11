use barm_proc::stringify_service_ref;

macro_rules! asd {
    ($($interface:tt)+) => {stringify_service_ref![$($interface)+]};
}

#[test]
fn test_stringify_service_def() {
    let asd = asd![dyn ExampleService<dyn Asd + Send> + Send];
    assert_eq!(asd, "ExampleService<Asd>")
}