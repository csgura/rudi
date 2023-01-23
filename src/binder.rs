use std::{
    any::TypeId,
    collections::HashMap,
    marker::PhantomData,
    sync::{Arc, Mutex},
};

use crate::{
    binding::Binding,
    provider::{Constructor, ConstructorProvider, SingletonProvider},
    Provider,
};

#[derive(Clone, Default)]
pub struct Binder {
    pub(crate) binds: Arc<Mutex<HashMap<TypeId, Binding>>>,
    pub(crate) overridable: Arc<Mutex<HashMap<TypeId, Binding>>>,
}

impl Binder {
    pub fn new() -> Binder {
        return Binder::default();
    }

    pub fn bind<T>(&self) -> BindTo<T>
    where
        T: 'static,
    {
        let type_name = std::any::type_name::<T>().into();
        BindTo {
            binder: self.clone(),
            type_id: TypeId::of::<T>(),
            type_name,
            phantom: PhantomData,
        }
    }

    pub(crate) fn merge(&mut self, other: &Binder) {
        let mut this_map = self.binds.lock().unwrap();
        let other_map = other.binds.lock().unwrap();
        other_map.iter().for_each(|(key, value)| {
            if !this_map.contains_key(key) {
                this_map.insert(key.clone(), value.clone());
            }
        })
    }

    pub(crate) fn merge_overridable(&mut self, other: &Binder) {
        let mut this_map = self.overridable.lock().unwrap();
        let other_map = other.binds.lock().unwrap();
        other_map.iter().for_each(|(key, value)| {
            if !this_map.contains_key(key) {
                this_map.insert(key.clone(), value.clone());
            }
        })
    }
}

pub struct BindTo<T: ?Sized> {
    binder: Binder,
    type_id: TypeId,
    type_name: String,
    phantom: PhantomData<T>,
}

impl<T: ?Sized> BindTo<T> {
    // pub fn to_provider<P>( & self,  p : P) where T : 'static + Sized , P : Provider<T> + 'static{

    //     let prov : BoxedProvider = BoxedProvider(Arc::new(p));

    //     let mut m = self.binder.binds.lock().unwrap();

    //     m.insert(self.typeId, prov);

    // }

    pub fn to_provider_dyn(&self, p: Arc<dyn Provider<T>>)
    where
        T: 'static + Sized,
    {
        let prov: Binding = Binding::new(self.type_name.clone(), Arc::new(p));

        let mut m = self.binder.binds.lock().unwrap();

        if m.contains_key(&self.type_id) {
            panic!("duplicated binding {}", self.type_name);
        }

        m.insert(self.type_id, prov);
    }

    pub fn to_singleton(&self, single: T)
    where
        T: 'static + Clone,
    {
        let p = SingletonProvider(single);

        let b: Arc<dyn Provider<T>> = Arc::new(p);

        self.to_provider_dyn(b)
    }

    pub fn to_constructor<A, C>(&self, c: C)
    where
        C: Constructor<A, T> + 'static,
        T: Sized + 'static,
        A: 'static,
    {
        let p: ConstructorProvider<A, T, C> = ConstructorProvider {
            constructor: c,
            pa: PhantomData,
            pt: PhantomData,
        };

        let b: Arc<dyn Provider<T>> = Arc::new(p);

        self.to_provider_dyn(b)
    }
}
