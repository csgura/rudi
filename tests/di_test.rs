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
impl AbstractModule for HelloModule {
    fn config( binder : &mut rudi::Binder ) {
        binder.bind::<Box<dyn Hello>>().to_provider(|i| {
            Box::new(HelloWorld{})
        });
    }
} 

#[test]
fn bind_test() {



}