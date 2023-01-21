use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{AbstractModule, Binder, Injector};

#[derive(Default)]
pub struct Implements {
    implements: HashMap<String, Arc<dyn AbstractModule>>,
}

impl Implements {
    pub fn new() -> Implements {
        Default::default()
    }
    pub fn add_implement<M: AbstractModule + 'static>(&mut self, name: String, module: M) {
        self.implements.insert(name, Arc::new(module));
    }

    pub fn has_implement(&mut self, name: String) -> bool {
        self.implements.contains_key(&name)
    }

    pub fn get_implement(&mut self, name: String) -> Option<Arc<dyn AbstractModule>> {
        return self.implements.get(&name).map(|x| x.clone());
    }

    pub fn add_implements(&mut self, other: &Implements) {
        other.implements.iter().for_each(|(key, value)| {
            self.implements.insert(key.clone(), value.clone());
        })
    }

    pub fn new_injector(&self, enabled: Vec<String>) -> Injector {
        let mut binder = Binder::new();

        enabled.iter().for_each(|name| {
            if let Some(module) = self.implements.get(name) {
                module.config(&mut binder);
            }
        });

        Injector {
            binds: binder,
            loop_checker: Default::default(),
        }
    }
}
