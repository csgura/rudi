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
