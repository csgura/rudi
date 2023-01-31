use std::sync::Arc;

use rudi::{bind_dyn_constructor, get_instance_dyn, AbstractModule, BindFunc, Binder, Implements};

trait A {}

trait B {}

trait C {}

struct AImpl {}

struct BImpl {}

struct CImpl {}

impl A for AImpl {}
impl B for BImpl {}
impl C for CImpl {}

fn new_a(_c: Arc<dyn C>) -> impl A {
    AImpl {}
}

fn new_b(_c: Arc<dyn A>) -> impl B {
    BImpl {}
}

fn new_c(_c: Arc<dyn B>) -> impl C {
    CImpl {}
}

struct LoopModule;

impl AbstractModule for LoopModule {
    fn config(&self, binder: &mut rudi::Binder) {
        bind_dyn_constructor!(binder, A, new_a);
        bind_dyn_constructor!(binder, B, new_b);
        bind_dyn_constructor!(binder, C, new_c);
    }
}

#[test]
#[should_panic]
fn loop_test() {
    let mut im = Implements::new();
    im.add_implement("hello".into(), LoopModule);

    let i = im.new_injector(vec!["hello".into()]);

    let _ins = get_instance_dyn!(i, C);
}

struct DupModule;

impl AbstractModule for DupModule {
    fn config(&self, binder: &mut rudi::Binder) {
        bind_dyn_constructor!(binder, A, new_a);
        bind_dyn_constructor!(binder, A, new_a);
    }
}

#[test]
#[should_panic]
fn dup_test() {
    let mut im = Implements::new();
    im.add_implement("hello".into(), DupModule);

    let i = im.new_injector(vec!["hello".into()]);

    let _ins = get_instance_dyn!(i, A);
}

fn not_binded_module(binder: &mut Binder) {
    bind_dyn_constructor!(binder, C, new_c);
}

#[test]
#[should_panic]
fn not_binded_test() {
    let mut im = Implements::new();
    im.add_implement("hello".into(), BindFunc(not_binded_module));

    let i = im.new_injector(vec!["hello".into()]);

    let _ins = get_instance_dyn!(i, C);
}
