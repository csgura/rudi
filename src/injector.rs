use std::{any::TypeId, collections::HashSet};

use crate::{binding::Binding, provider::Constructor, Binder};

#[derive(Clone, Default)]
pub(crate) struct LoopChecker {
    pub(crate) visited: HashSet<String>,
    pub(crate) stack: Vec<String>,
}

impl LoopChecker {
    pub(crate) fn path(&self) -> String {
        self.stack.join(" -> ").into()
    }
    pub(crate) fn visit(&self, name: String) -> LoopChecker {
        let mut visited = self.visited.clone();
        visited.insert(name.clone());

        let mut stack = self.stack.clone();
        stack.push(name.clone());
        LoopChecker { visited, stack }
    }
}

#[derive(Clone)]
pub struct Injector {
    pub(crate) binds: Binder,
    pub(crate) loop_checker: LoopChecker,
}

impl Injector {
    fn get_overridable_bind<T: 'static>(&self) -> Option<Binding> {
        let typeid = TypeId::of::<T>();

        let binder = self.binds.overridable.lock().unwrap();

        let bind = binder.get(&typeid);

        bind.map(|x| x.clone())
        //bind.and_then(|p| p.downcast::<T>().clone())
    }

    fn get_bind<T: 'static>(&self) -> Option<Binding> {
        let typeid = TypeId::of::<T>();

        let binder = self.binds.binds.lock().unwrap();

        let bind = binder.get(&typeid);

        bind.map(|x| x.clone())
            .or_else(|| self.get_overridable_bind::<T>())
        //bind.and_then(|p| p.downcast::<T>().clone())
    }

    pub fn get_instance<T>(&self) -> Option<T>
    where
        T: 'static + Clone,
    {
        let b = self.get_bind::<T>();

        b.map(|x| x.get_instance::<T>(self))
    }

    pub fn inject_and_call<A, R, C>(&self, c: C) -> R
    where
        C: Constructor<A, R>,
    {
        c.new(self)
    }
}
