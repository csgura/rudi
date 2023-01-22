use std::sync::Arc;

use crate::Binder;

pub trait AbstractModule {
    fn config(&self, binder: &mut Binder);
}

#[derive(Clone)]
pub struct CombinedModule {
    modules: Vec<Arc<dyn AbstractModule>>,
}

impl AbstractModule for CombinedModule {
    fn config(&self, binder: &mut Binder) {
        self.modules.iter().for_each(|m| m.config(binder))
    }
}

impl CombinedModule {
    pub fn new(modules: Vec<Arc<dyn AbstractModule>>) -> CombinedModule {
        CombinedModule { modules: modules }
    }
}

#[macro_export]
macro_rules! combine_module {
    ($($m:expr),*) => {
        $crate::CombinedModule::new(vec![$(Arc::new($m),)*])
    };
}
