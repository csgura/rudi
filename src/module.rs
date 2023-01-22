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

pub struct OverridableModule {
    overriden: Vec<Arc<dyn AbstractModule>>,
}

impl AbstractModule for OverridableModule {
    fn config(&self, binder: &mut Binder) {
        self.overriden.iter().for_each(|m| m.config(binder))
    }
}

pub struct OverridedModule {
    overriden: Vec<Arc<dyn AbstractModule>>,
    overrides: Vec<Arc<dyn AbstractModule>>,
}

impl AbstractModule for OverridedModule {
    fn config(&self, binder: &mut Binder) {
        todo!()
    }
}

impl OverridableModule {
    fn with(&self, overrides: Vec<Arc<dyn AbstractModule>>) -> OverridedModule {
        OverridedModule {
            overriden: self.overriden.clone(),
            overrides,
        }
    }
}
