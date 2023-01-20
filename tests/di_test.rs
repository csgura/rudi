use std::sync::Arc;

use rudi::AbstractModule;


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
    fn config( binder : &mut rudi::Binder ) {
        binder.bind::<Arc<dyn Hello>>().to_provider(|i| {
            Arc::new(HelloWorld{})
        });

        binder.bind::<Arc<dyn Hello>>().to_constructor(new_hello);
    }
} 

#[test]
fn bind_test() {



}