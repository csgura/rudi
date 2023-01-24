use std::sync::Arc;

use rudi::{bind, bind_dyn, AbstractModule, BindFunc, Binder, Implements};

pub struct HelloModule {}

trait Hello {
    fn hello(&self) -> String;
}

trait Dep1 {
    fn message(&self) -> String;
}

#[derive(Clone)]
struct Dep1Impl {
    msg: String,
}

impl Dep1 for Dep1Impl {
    fn message(&self) -> String {
        self.msg.clone()
    }
}

#[derive(Clone)]
struct HelloWorld {}
impl Hello for HelloWorld {
    fn hello(&self) -> String {
        "hello world".into()
    }
}

// fn new_hello(d1: Arc<dyn Dep1>) -> HelloWorld {
//     println!("d1.msg = {}", d1.message());
//     HelloWorld {}
// }

fn new_hello(d1: Arc<dyn Dep1>) -> Arc<dyn Hello> {
    println!("d1.msg = {}", d1.message());
    Arc::new(HelloWorld {})
}

impl AbstractModule for HelloModule {
    fn config(&self, binder: &mut rudi::Binder) {
        // binder.bind::<Arc<dyn Hello>>().to_provider(|i| {
        //     Arc::new(HelloWorld{})
        // });

        binder
            .bind::<Arc<dyn Dep1>>()
            .to_singleton(Arc::new(Dep1Impl {
                msg: "hello".into(),
            }));

        bind_dyn!(binder, Hello).to_constructor(new_hello);

        //binder.bind::<Arc<dyn Hello>>().to_constructor(new_hello);
    }
}

#[test]
fn bind_test() {
    let mut im = Implements::new();
    im.add_implement("hello".into(), HelloModule {});

    let i = im.new_injector(vec!["hello".into()]);

    let ins = i.get_instance::<Arc<dyn Hello>>();

    assert_eq!(ins.is_some(), true);

    let ins = i.get_instance::<Arc<dyn Hello>>();

    assert_eq!(ins.is_some(), true);
}

struct OtherModule {}

impl AbstractModule for OtherModule {
    fn config(&self, binder: &mut rudi::Binder) {
        binder.bind::<u32>().to_singleton(20);
    }
}

#[test]
fn combine_test() {
    let mut im = Implements::new();

    let m = rudi::combine_module!(HelloModule {}, OtherModule {});
    im.add_implement("hello".into(), m);

    let i = im.new_injector(vec!["hello".into()]);

    let ins = i.get_instance::<Arc<dyn Hello>>();

    assert_eq!(ins.is_some(), true);

    let ins = i.get_instance::<u32>();

    assert_eq!(ins.is_some(), true);
}

fn default_module(binder: &mut Binder) {
    binder.bind::<Dep1Impl>().to_singleton(Dep1Impl {
        msg: "default".into(),
    });

    binder.bind::<u32>().to_singleton(42);
}

fn override_module(binder: &mut Binder) {
    bind!(binder, Dep1Impl).to_singleton(Dep1Impl {
        msg: "override".into(),
    });
}

#[test]
fn default_test() {
    let mut im = Implements::new();

    let m = rudi::overridable_module!(BindFunc(default_module));
    im.add_implement("default".into(), m);
    im.add_implement("override".into(), BindFunc(override_module));

    let i = im.new_injector(vec!["default".into(), "override".into()]);

    let ins = i.get_instance::<Dep1Impl>();

    assert_eq!(ins.is_some(), true);
    assert_eq!(ins.unwrap().msg, String::from("override"));

    let ins = i.get_instance::<u32>();

    assert_eq!(ins.is_some(), true);
    assert_eq!(ins.unwrap(), 42);
}

fn hello_eager() -> Arc<dyn Hello> {
    println!("hello eager");
    Arc::new(HelloWorld {})
}

fn eager_module(binder: &mut Binder) {
    bind!(binder, Arc<dyn Hello>)
        .to_constructor(hello_eager)
        .as_eager();
}

#[test]
fn eager_test() {
    let mut im = Implements::new();

    im.add_implement("eager".into(), BindFunc(eager_module));

    let _i = im.new_injector(vec!["eager".into()]);
}
