use std::{collections::HashMap, sync::Arc};

use crate::{AbstractModule, Binder, Injector};

#[derive(Default, Clone)]
pub struct Implements {
    anonymous_module: Vec<Arc<dyn AbstractModule>>,
    named_module: HashMap<String, Arc<dyn AbstractModule>>,
}

impl Implements {
    pub fn new() -> Implements {
        Default::default()
    }

    pub fn add_bind<M: AbstractModule + 'static>(&mut self, module: M) {
        self.anonymous_module.push(Arc::new(module));
    }

    pub fn add_implement<M: AbstractModule + 'static, S>(&mut self, name: S, module: M)
    where
        S: AsRef<str>,
    {
        self.named_module
            .insert(String::from(name.as_ref()), Arc::new(module));
    }

    pub fn has_implement(&mut self, name: String) -> bool {
        self.named_module.contains_key(&name)
    }

    pub fn get_implement(&mut self, name: String) -> Option<Arc<dyn AbstractModule>> {
        return self.named_module.get(&name).map(|x| x.clone());
    }

    pub fn add_implements(&mut self, other: &Implements) {
        other.named_module.iter().for_each(|(key, value)| {
            self.named_module.insert(key.clone(), value.clone());
        })
    }

    pub fn new_injector(&self, enabled: Vec<String>) -> Injector {
        let mut binder = Binder::new();

        for m in &self.anonymous_module {
            m.config(&mut binder);
        }

        enabled.iter().for_each(|name| {
            if let Some(module) = self.named_module.get(name) {
                module.config(&mut binder);
            } else {
                panic!("module {} not exists", name);
            }
        });

        let ret = Injector {
            binds: binder,
            loop_checker: Default::default(),
        };

        let eager = ret.binds.get_eager_bindings();

        eager.into_iter().for_each(|b| {
            println!("start eager singleton {}", b.type_name());
            b.prepare_instance(&ret)
        });

        ret
    }
}
