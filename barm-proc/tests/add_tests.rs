use barm_proc::add_traits;

#[test]
fn test_add_traits() {
    let x: Box<add_traits![dyn ToString, core::marker::Sync]> = Box::new(1);
    let x: Box<dyn ToString + Sync> = x;
    assert_eq!(x.to_string(), "1");

    trait Asd<T> {
        fn asd(&self, t: T) -> T;
    }
    struct AsdImpl;
    impl Asd<i32> for AsdImpl {
        fn asd(&self, t: i32) -> i32 {
            t
        }
    }
    let asd: Box<dyn Asd<i32> + Sync> = Box::new(AsdImpl);
}