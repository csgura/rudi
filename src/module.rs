use crate::Binder;

pub trait AbstractModule {
    fn config(&self, binder: &mut Binder);
}

pub struct CombinedModule {
    modules: Vec<Box<dyn AbstractModule>>,
}

impl AbstractModule for CombinedModule {
    fn config(&self, binder: &mut Binder) {
        self.modules.iter().for_each(|m| m.config(binder))
    }
}

impl CombinedModule {
    pub fn new(modules: Vec<Box<dyn AbstractModule>>) -> CombinedModule {
        CombinedModule { modules: modules }
    }
}

#[macro_export]
macro_rules! combine_module {
    ($($m:expr),*) => {
        $crate::CombinedModule::new(vec![$(Box::new($m),)*])
    };
}
