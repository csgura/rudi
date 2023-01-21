use std::{
    any::{Any, TypeId},
    collections::{HashMap, HashSet},
    marker::PhantomData,
    sync::{Arc, Mutex},
};

mod implements;

pub use implements::Implements;

pub trait Provider<T> {
    fn provide(&self, injector: &Injector) -> T;
}

#[derive(Clone, Default)]
struct LoopChecker {
    visited: HashSet<String>,
    stack: Vec<String>,
}

impl LoopChecker {
    fn path(&self) -> String {
        self.stack.join(" -> ").into()
    }
    fn visit(&self, name: String) -> LoopChecker {
        let mut visited = self.visited.clone();
        visited.insert(name.clone());

        let mut stack = self.stack.clone();
        stack.push(name.clone());
        LoopChecker { visited, stack }
    }
}

#[derive(Clone)]
pub struct Injector {
    binds: Binder,
    loop_checker: LoopChecker,
}

trait Singleton {
    type ProvidedType;
    fn get(&self) -> Arc<Self::ProvidedType>;
}

#[derive(Clone, Default)]
pub struct Binder {
    binds: Arc<Mutex<HashMap<TypeId, Binding>>>,
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

impl<T, F> Provider<T> for F
where
    F: for<'a> Fn(&'a Injector) -> T,
{
    fn provide(&self, injector: &Injector) -> T {
        self(injector)
    }
}

pub trait Constructor<A, R> {
    fn new(&self, injector: &Injector) -> R;
}

struct ConstructorProvider<A, T, C: Constructor<A, T>> {
    constructor: C,
    pa: PhantomData<A>,
    pt: PhantomData<T>,
}

impl<A, T, C> Provider<T> for ConstructorProvider<A, T, C>
where
    C: Constructor<A, T>,
{
    fn provide(&self, injector: &Injector) -> T {
        self.constructor.new(injector)
    }
}

struct SingletonProvider<T: Clone>(T);

impl<T: Clone> Provider<T> for SingletonProvider<T> {
    fn provide(&self, injector: &Injector) -> T {
        self.0.clone()
    }
}

// impl<A,T,C> Provider for C where
// C : Constructor<A,T> {
//     type ProvidedType = T;

//     fn provide(&self,  injector : &Injector ) -> Self::ProvidedType {
//         self(injector)
//     }
// }

macro_rules! impl_handler {
    (
        [$($ty:ident),*], $last:ident
    ) => {
        #[allow(non_snake_case, unused_mut)]
        impl <F,$($ty,)* $last> Constructor<($($ty,)*), $last> for F
        where F : Fn($($ty,)*) -> $last,
        $(
            $ty : 'static + Clone,
        )*
         {
            fn new(&self, injector : &Injector) -> $last {
                $(
                    let $ty = injector.get_instance::<$ty>().unwrap();
                )*
                let res = self($($ty,)*);
                res
            }
        }
    }
}

macro_rules! all_the_tuples {
    ($name:ident) => {
        $name!([], T1);
        $name!([T1], T2);
        $name!([T1, T2], T3);
        $name!([T1, T2, T3], T4);
        $name!([T1, T2, T3, T4], T5);
        $name!([T1, T2, T3, T4, T5], T6);
        $name!([T1, T2, T3, T4, T5, T6], T7);
        $name!([T1, T2, T3, T4, T5, T6, T7], T8);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8], T9);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9], T10);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10], T11);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11], T12);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12], T13);
        $name!(
            [T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13],
            T14
        );
        $name!(
            [T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14],
            T15
        );
        $name!(
            [T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15],
            T16
        );
    };
}

all_the_tuples!(impl_handler);

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

pub trait AbstractModule {
    fn config(&self, binder: &mut Binder);
}

#[derive(Clone)]
struct Binding {
    type_name: String,
    provider: Arc<dyn Any>,
    instance: Arc<Mutex<Option<Arc<dyn Any>>>>,
}

impl Binding {
    fn new(type_name: String, provider: Arc<dyn Any>) -> Binding {
        Binding {
            type_name,
            provider,
            instance: Arc::new(Mutex::new(None)),
        }
    }

    fn downcast<T: 'static>(&self) -> Option<Arc<dyn Provider<T>>> {
        self.provider
            .downcast_ref::<Arc<dyn Provider<T>>>()
            .map(|x| x.clone())
    }

    fn get_instance<T: 'static + Clone>(&mut self, injector: &Injector) -> T {
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

impl Injector {
    fn get_bind<T: 'static>(&self) -> Option<Binding> {
        let typeid = TypeId::of::<T>();

        let binder = self.binds.binds.lock().unwrap();

        let bind = binder.get(&typeid);

        bind.map(|x| x.clone())
        //bind.and_then(|p| p.downcast::<T>().clone())
    }

    pub fn get_instance<T>(&self) -> Option<T>
    where
        T: 'static + Clone,
    {
        let b = self.get_bind::<T>();

        b.map(|mut x| x.get_instance::<T>(self))
    }
}
