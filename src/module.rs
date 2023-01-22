use crate::Binder;

pub trait AbstractModule {
    fn config(&self, binder: &mut Binder);
}
