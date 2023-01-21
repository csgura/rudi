use std::sync::Arc;

use rudi::{AbstractModule, Implements};


pub struct HelloModule {

}

trait Hello {

    fn hello(&self) -> String;
}

trait Dep1 {
    fn message(&self) -> String;
}

struct Dep1Impl {
    msg : String
} 

impl Dep1 for Dep1Impl {
    fn message(&self) -> String {
        self.msg.clone()
    }
}

struct HelloWorld {

}
impl Hello for HelloWorld {
    fn hello(&self) -> String {
        "hello world".into()
    }
}

fn new_hello( d1 : Arc<dyn Dep1> ) -> Arc<dyn Hello> {
    println!("d1.msg = {}", d1.message());
    Arc::new(HelloWorld{})
}

impl AbstractModule for HelloModule {
    fn config( &self, binder : &mut rudi::Binder ) {
        // binder.bind::<Arc<dyn Hello>>().to_provider(|i| {
        //     Arc::new(HelloWorld{})
        // });

        binder.bind::<Arc<dyn Dep1>>().to_singleton(Arc::new(Dep1Impl{msg : "hello".into()}));
        binder.bind::<Arc<dyn Hello>>().to_constructor(new_hello);
    }
} 

#[test]
fn bind_test() {

    let mut im = Implements::new();
    im.add_implement("hello".into(), HelloModule{});

    let i = im.new_injector(vec!["hello".into()]);

    let ins = i.get_instance::<Arc<dyn Hello>>();

    assert_eq!(ins.is_some(), true);

    let ins = i.get_instance::<Arc<dyn Hello>>();

    assert_eq!(ins.is_some(), true);
}