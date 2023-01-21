use std::sync::Arc;

use rudi::{AbstractModule, Implements};


pub struct HelloModule {

}

trait Hello {

    fn hello(&self) -> String;
}


struct HelloWorld {

}
impl Hello for HelloWorld {
    fn hello(&self) -> String {
        "hello world".into()
    }
}

fn new_hello( a : Arc<String>, b : Arc<u32> ) -> Arc<dyn Hello> {
    Arc::new(HelloWorld{})
}

impl AbstractModule for HelloModule {
    fn config( &self, binder : &mut rudi::Binder ) {
        // binder.bind::<Arc<dyn Hello>>().to_provider(|i| {
        //     Arc::new(HelloWorld{})
        // });

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
}