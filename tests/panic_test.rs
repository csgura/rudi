use std::sync::Arc;

use rudi::{AbstractModule, BindFunc, Binder, Implements};

trait A {}

trait B {}

trait C {}

struct AImpl {}

struct BImpl {}

struct CImpl {}

impl A for AImpl {}
impl B for BImpl {}
impl C for CImpl {}

fn new_a(_c: Arc<dyn C>) -> Arc<dyn A> {
    Arc::new(AImpl {})
}

fn new_b(_c: Arc<dyn A>) -> Arc<dyn B> {
    Arc::new(BImpl {})
}

fn new_c(_c: Arc<dyn B>) -> Arc<dyn C> {
    Arc::new(CImpl {})
}

struct LoopModule {}

impl AbstractModule for LoopModule {
    fn config(&self, binder: &mut rudi::Binder) {
        binder.bind::<Arc<dyn A>>().to_constructor(new_a);
        binder.bind::<Arc<dyn B>>().to_constructor(new_b);
        binder.bind::<Arc<dyn C>>().to_constructor(new_c);
    }
}

#[test]
#[should_panic]
fn loop_test() {
    let mut im = Implements::new();
    im.add_implement("hello".into(), LoopModule {});

    let i = im.new_injector(vec!["hello".into()]);

    let _ins = i.get_instance::<Arc<dyn C>>();
}

struct DupModule {}

impl AbstractModule for DupModule {
    fn config(&self, binder: &mut rudi::Binder) {
        binder.bind::<Arc<dyn A>>().to_constructor(new_a);
        binder.bind::<Arc<dyn A>>().to_constructor(new_a);
    }
}

#[test]
#[should_panic]
fn dup_test() {
    let mut im = Implements::new();
    im.add_implement("hello".into(), DupModule {});

    let i = im.new_injector(vec!["hello".into()]);

    let _ins = i.get_instance::<Arc<dyn A>>();
}

fn not_binded_module(binder: &mut Binder) {
    binder.bind::<Arc<dyn C>>().to_constructor(new_c);
}

#[test]
#[should_panic]
fn not_binded_test() {
    let mut im = Implements::new();
    im.add_implement("hello".into(), BindFunc(not_binded_module));

    let i = im.new_injector(vec!["hello".into()]);

    let _ins = i.get_instance::<Arc<dyn C>>();
}
