use std::sync::Arc;

use rudi::{AbstractModule, Implements};

struct LoopModule {}

trait A {}

trait B {}

trait C {}

struct AImpl {}

struct BImpl {}

struct CImpl {}

impl A for AImpl {}
impl B for BImpl {}
impl C for CImpl {}

fn new_A(c: Arc<dyn C>) -> Arc<dyn A> {
    Arc::new(AImpl {})
}

fn new_B(c: Arc<dyn A>) -> Arc<dyn B> {
    Arc::new(BImpl {})
}

fn new_C(c: Arc<dyn B>) -> Arc<dyn C> {
    Arc::new(CImpl {})
}

impl AbstractModule for LoopModule {
    fn config(&self, binder: &mut rudi::Binder) {
        binder.bind::<Arc<dyn A>>().to_constructor(new_A);
        binder.bind::<Arc<dyn B>>().to_constructor(new_B);
        binder.bind::<Arc<dyn C>>().to_constructor(new_C);
    }
}

#[test]
#[should_panic]
fn loop_test() {
    let mut im = Implements::new();
    im.add_implement("hello".into(), LoopModule {});

    let i = im.new_injector(vec!["hello".into()]);

    let ins = i.get_instance::<Arc<dyn C>>();
}
