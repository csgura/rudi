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

    pub(crate) fn get_eager_bindings(&self) -> Vec<Binding> {
        let m = self.binds.lock().unwrap();

        m.iter()
            .map(|t| t.1)
            .filter(|b| b.is_eager)
            .map(|b| b.clone())
            .collect()
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

pub struct BindOption {
    binder: Binder,
    type_id: TypeId,
}

impl BindOption {
    pub fn as_eager(self) -> BindOption {
        {
            let mut m = self.binder.binds.lock().unwrap();
            let b = m.get_mut(&self.type_id);
            b.into_iter().for_each(|b| b.set_as_eager())
        }
        self
    }
}

impl<T: ?Sized> BindTo<T> {
    // pub fn to_provider<P>( & self,  p : P) where T : 'static + Sized , P : Provider<T> + 'static{

    //     let prov : BoxedProvider = BoxedProvider(Arc::new(p));

    //     let mut m = self.binder.binds.lock().unwrap();

    //     m.insert(self.typeId, prov);

    // }

    pub fn to_provider_dyn(self, p: Arc<dyn Provider<T>>) -> BindOption
    where
        T: 'static + Sized,
    {
        let binder = self.binder;
        let type_name = self.type_name;
        let type_id = self.type_id;

        let prov: Binding = Binding::new(type_name.clone(), Arc::new(p));

        {
            let mut m = binder.binds.lock().unwrap();

            if m.contains_key(&type_id) {
                panic!("duplicated binding {}", type_name);
            }
            m.insert(type_id, prov);
        }

        BindOption {
            binder: binder,
            type_id: type_id,
        }
    }

    pub fn to_singleton(self, single: T) -> BindOption
    where
        T: 'static + Clone,
    {
        let p = SingletonProvider(single);

        let b: Arc<dyn Provider<T>> = Arc::new(p);

        self.to_provider_dyn(b)
    }

    pub fn to_constructor<A, C>(self, c: C) -> BindOption
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
