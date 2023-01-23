use std::sync::Arc;

use crate::Binder;

pub trait AbstractModule {
    fn config(&self, binder: &mut Binder);
}

#[derive(Clone)]
pub struct BindFunc(pub fn(&mut Binder));

impl AbstractModule for BindFunc {
    fn config(&self, binder: &mut Binder) {
        self.0(binder)
    }
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

#[macro_export]
macro_rules! overridable_module {
    ($($m:expr),*) => {
        $crate::OverridableModule::new(vec![$(Arc::new($m),)*])
    };
}

pub struct OverridableModule {
    overriden: Vec<Arc<dyn AbstractModule>>,
}

impl OverridableModule {
    pub fn new(modules: Vec<Arc<dyn AbstractModule>>) -> OverridableModule {
        OverridableModule { overriden: modules }
    }
}
impl AbstractModule for OverridableModule {
    fn config(&self, binder: &mut Binder) {
        let mut ob = Binder::new();
        self.overriden.iter().for_each(|m| m.config(&mut ob));
        binder.merge_overridable(&ob);
    }
}

pub struct OverridedModule {
    overriden: Vec<Arc<dyn AbstractModule>>,
    overrides: Vec<Arc<dyn AbstractModule>>,
}

impl AbstractModule for OverridedModule {
    fn config(&self, binder: &mut Binder) {
        self.overrides.iter().for_each(|m| m.config(binder));

        let mut ob = Binder::new();
        self.overriden.iter().for_each(|m| m.config(&mut ob));

        binder.merge(&ob);
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
