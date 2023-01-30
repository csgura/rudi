use std::sync::Arc;

use rudi::{bind_dyn, bind_dyn_constructor, AbstractModule, Implements};

struct MyModule;

trait Hello {
    fn hello(&self) -> String;
}

struct HelloImpl {
    msg: String,
}
impl Hello for HelloImpl {
    fn hello(&self) -> String {
        self.msg.clone()
    }
}

trait World {
    fn world(&self) -> String;
}

struct WorldImpl {
    hello: Arc<dyn Hello>,
}

impl World for WorldImpl {
    fn world(&self) -> String {
        self.hello.hello()
    }
}

impl WorldImpl {
    fn new(h: Arc<dyn Hello>) -> Self {
        WorldImpl { hello: h }
    }
}

impl Drop for WorldImpl {
    fn drop(&mut self) {
        println!("drop worldImpl");
    }
}

impl AbstractModule for MyModule {
    fn config(&self, binder: &mut rudi::Binder) {
        bind_dyn_constructor!(binder, World, WorldImpl::new);
        bind_dyn!(binder, Hello).to_singleton(Arc::new(HelloImpl {
            msg: "hello".into(),
        }));
    }
}

fn is_send<T: Send>(_: &T) {}
#[test]
fn drop_test() {
    let mut i = Implements::new();
    i.add_bind(MyModule {});

    let app = i.new_injector(Vec::new());
    let world = app.get_instance::<Arc<dyn World>>().unwrap();
    world.world();

    is_send(&app);

    drop(app);
}
