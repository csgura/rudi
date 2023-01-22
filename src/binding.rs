use std::{
    any::Any,
    sync::{Arc, Mutex},
};

use crate::{Injector, Provider};

#[derive(Clone)]
pub(crate) struct Binding {
    type_name: String,
    provider: Arc<dyn Any>,
    instance: Arc<Mutex<Option<Arc<dyn Any>>>>,
}

impl Binding {
    pub(crate) fn new(type_name: String, provider: Arc<dyn Any>) -> Binding {
        Binding {
            type_name,
            provider,
            instance: Arc::new(Mutex::new(None)),
        }
    }

    pub(crate) fn downcast<T: 'static>(&self) -> Option<Arc<dyn Provider<T>>> {
        self.provider
            .downcast_ref::<Arc<dyn Provider<T>>>()
            .map(|x| x.clone())
    }

    pub(crate) fn get_instance<T: 'static + Clone>(&mut self, injector: &Injector) -> T {
        if injector.loop_checker.visited.contains(&self.type_name) {
            panic!("loop detected. path = {}", injector.loop_checker.path());
        }

        let mut guard = self.instance.lock().unwrap();

        if let Some(ret) = guard.as_ref() {
            return ret.downcast_ref::<T>().unwrap().clone();
        }

        let p = self.downcast::<T>().unwrap();

        let checked = Injector {
            binds: injector.binds.clone(),
            loop_checker: injector.loop_checker.visit(self.type_name.clone()),
        };

        let ins = p.provide(&checked);

        *guard = Some(Arc::new(ins.clone()));

        ins
    }
}
