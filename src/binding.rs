use std::{
    any::Any,
    sync::{Arc, Mutex},
};

use crate::{Injector, ProviderAny};

#[derive(Clone)]
pub(crate) struct Binding {
    type_name: String,
    provider: Arc<dyn Any>,
    instance: Arc<Mutex<Option<Box<dyn Any>>>>,
    pub(crate) is_eager: bool,
}

impl Binding {
    pub(crate) fn set_as_eager(&mut self) {
        self.is_eager = true;
    }
    pub(crate) fn new(type_name: String, provider: Arc<dyn Any>) -> Binding {
        Binding {
            type_name,
            provider,
            instance: Arc::new(Mutex::new(None)),
            is_eager: false,
        }
    }

    pub(crate) fn downcast(&self) -> Option<Arc<dyn ProviderAny>> {
        self.provider
            .downcast_ref::<Arc<dyn ProviderAny>>()
            .map(|x| x.clone())
    }

    pub(crate) fn prepare_instance(&self, injector: &Injector)  {
        if injector.loop_checker.visited.contains(&self.type_name) {
            panic!("loop detected. path = {}", injector.loop_checker.path());
        }

        let mut guard = self.instance.lock().unwrap();

        if let Some(_) = guard.as_ref() {
            return;
        }

        let p = self.downcast().unwrap();

        let checked = Injector {
            binds: injector.binds.clone(),
            loop_checker: injector.loop_checker.visit(self.type_name.clone()),
        };

        let ins = p.provide_any(&checked);

        *guard = Some(ins);
    }

    pub(crate) fn get_instance<T: 'static + Clone>(&self, injector: &Injector) -> T {
        
        self.prepare_instance(injector);
        let guard = self.instance.lock().unwrap();

        if let Some(ret) = guard.as_ref() {
            return ret.downcast_ref::<T>().unwrap().clone();
        } else {
            panic!("impossible");
        }
    }
}
