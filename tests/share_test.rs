use std::{pin::Pin, time::Duration};

use futures::{future::Shared, Future, FutureExt};
use rudi::{bind, get_instance, AbstractModule, Implements};

struct FutureModule;

#[derive(Clone)]
struct Promise<T: Clone> {
    f: Shared<Pin<Box<dyn Future<Output = T> + Send>>>,
}

impl<T: Clone> Promise<T> {
    fn new_from_pin(f: Pin<Box<dyn Future<Output = T> + Send>>) -> Self {
        let shared = f.shared();
        Promise { f: shared }
    }

    fn new<F>(f: F) -> Self
    where
        F: Future<Output = T> + 'static + Send,
    {
        Self::new_from_pin(Box::pin(f))
    }

    async fn get(&mut self) -> T {
        let c = self.f.clone();
        c.await
    }
}

#[derive(Clone)]
struct Hello {
    fo: Promise<String>,
}

impl<'a> Hello {
    async fn hello(mut self) -> String {
        self.fo.get().await
    }
}

impl AbstractModule for FutureModule {
    fn config(&self, binder: &mut rudi::Binder) {
        bind!(binder, Hello).to_singleton(Hello {
            fo: Promise::new(async move { "Hello".into() }),
        });
    }
}

fn is_send<T: Send>(_: &T) {}
#[tokio::test]
async fn share_test() {
    let mut im = Implements::new();
    im.add_bind(FutureModule);

    let i = im.new_injector(Vec::new());
    let h = get_instance!(i, Hello).unwrap();

    is_send(&h);

    assert_eq!(h.hello().await, String::from("Hello"));

    // let f = async move {
    //     tokio::time::sleep(Duration::from_secs(5)).await;
    //     println!("after sleep");
    //     6
    // };
    // let s2 = f.shared();

    // println!("val = {}", s2.clone().await);
    // println!("val2 = {}", s2.clone().await);
    // println!("val3 = {}", s2.clone().await);
}
