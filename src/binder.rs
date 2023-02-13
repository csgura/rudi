use std::{
    any::TypeId,
    collections::HashMap,
    marker::PhantomData,
    sync::{Arc, Mutex},
};

use crate::{
    binding::{Binding, InterceptBinding},
    provider::{
        BoxedIntercept, BoxedProvider, Constructor, ConstructorProvider, InterceptProvider,
        InterceptProviderAny, Provider, SingletonProvider,
    },
    Injector, InterceptFunc, ProviderAny,
};

#[derive(Clone, Default)]
pub struct Binder {
    pub(crate) binds: Arc<Mutex<HashMap<TypeId, Binding>>>,
    pub(crate) overridable: Arc<Mutex<HashMap<TypeId, Binding>>>,
    pub(crate) intercepts: Arc<Mutex<HashMap<TypeId, Vec<InterceptBinding>>>>,
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

    pub fn intercept<T>(&self) -> Intercept<T>
    where
        T: 'static,
    {
        let type_name = std::any::type_name::<T>().into();
        Intercept {
            binder: self.clone(),
            type_id: TypeId::of::<T>(),
            type_name,
            phantom: PhantomData,
        }
    }

    pub(crate) fn get_eager_bindings(&self) -> Vec<Binding> {
        let m = self.binds.lock().unwrap();
        let m2 = self.overridable.lock().unwrap();

        let i1 = m
            .iter()
            .map(|t| t.1)
            .filter(|b| b.is_eager)
            .map(|b| b.clone());

        let i2 = m2
            .iter()
            .map(|t| t.1)
            .filter(|b| b.is_eager)
            .map(|b| b.clone());

        i1.chain(i2).collect()
    }

    pub(crate) fn add_interceptor(&mut self, type_id: TypeId, interceptor: InterceptBinding) {
        let mut m = self.intercepts.lock().unwrap();
        let opt = m.get_mut(&type_id);
        match opt {
            Some(list) => {
                list.push(interceptor);
            }
            None => {
                m.insert(type_id, vec![interceptor]);
            }
        };
    }

    pub(crate) fn merge(&mut self, other: &Binder) {
        {
            let mut this_map = self.binds.lock().unwrap();
            let other_map = other.binds.lock().unwrap();
            other_map.iter().for_each(|(key, value)| {
                if !this_map.contains_key(key) {
                    this_map.insert(key.clone(), value.clone());
                }
            });
        }
        {
            let other_map = other.intercepts.lock().unwrap();

            other_map.iter().for_each(|(key, value)| {
                value.iter().for_each(|i| {
                    self.add_interceptor(key.clone(), i.clone());
                })
            });
        }
    }

    pub(crate) fn merge_overridable(&mut self, other: &Binder) {
        {
            let mut this_map = self.overridable.lock().unwrap();
            let other_map = other.binds.lock().unwrap();
            other_map.iter().for_each(|(key, value)| {
                if !this_map.contains_key(key) {
                    this_map.insert(key.clone(), value.clone());
                }
            })
        }
        {
            let other_map = other.intercepts.lock().unwrap();

            other_map.iter().for_each(|(key, value)| {
                value.iter().for_each(|i| {
                    self.add_interceptor(key.clone(), i.clone());
                })
            });
        }
    }

    pub(crate) fn get_intercepts(&self, type_id: TypeId) -> Vec<InterceptBinding> {
        let m = self.intercepts.lock().unwrap();
        let l = m.get(&type_id);
        l.map(|x| x.clone()).unwrap_or_default()
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
    pub fn to_provider_dyn(self, p: Arc<dyn ProviderAny>) -> BindOption
    where
        T: 'static + Sized,
    {
        let binder = self.binder;
        let type_name = self.type_name;
        let type_id = self.type_id;

        let prov: Binding = Binding::new(type_id, type_name.clone(), p);

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

        let b: Arc<dyn ProviderAny> = Arc::new(p);

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

        let b: Arc<dyn ProviderAny> = Arc::new(p);

        self.to_provider_dyn(b)
    }

    pub fn to_provider<P>(self, p: P) -> BindOption
    where
        T: 'static + Sized,
        P: Provider<Provided = T> + 'static,
    {
        let b: Arc<dyn ProviderAny> = Arc::new(BoxedProvider { p });

        self.to_provider_dyn(b)
    }
}

pub struct Intercept<T: ?Sized> {
    binder: Binder,
    type_id: TypeId,
    type_name: String,
    phantom: PhantomData<T>,
}

impl<T: 'static> Intercept<T> {
    fn to_dyn(self, ip: Arc<dyn InterceptProviderAny>) {
        let mut m = self.binder.intercepts.lock().unwrap();
        let opt = m.get_mut(&self.type_id);
        if let Some(l) = opt {
            l.push(InterceptBinding {
                type_name: self.type_name,
                provider: ip,
            })
        } else {
            m.insert(
                self.type_id,
                vec![InterceptBinding {
                    type_name: self.type_name,
                    provider: ip,
                }],
            );
        }
    }

    pub fn to<P: 'static + InterceptProvider<Provided = T>>(self, ip: P) {
        self.to_dyn(Arc::new(BoxedIntercept(ip)))
    }

    pub fn to_func(self, ip: fn(&Injector, T) -> T) {
        self.to(InterceptFunc(ip))
    }
}
