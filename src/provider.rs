use std::{any::Any, marker::PhantomData, sync::Arc};

use crate::Injector;

pub trait ProviderAny {
    fn provide_any(&self, injector: &Injector) -> Box<dyn Any>;
}

pub trait Provider {
    type Provided;
    fn provide(&self, injector: &Injector) -> Self::Provided;
}

pub struct ProviderFunc<T>(pub fn(&Injector) -> T);
impl<T> Provider for ProviderFunc<T> {
    type Provided = T;

    fn provide(&self, injector: &Injector) -> Self::Provided {
        self.0(injector)
    }
}
pub(crate) struct BoxedProvider<T, P>
where
    P: Provider<Provided = T>,
{
    pub(crate) p: P,
}

impl<T, P> ProviderAny for BoxedProvider<T, P>
where
    P: Provider<Provided = T>,
    T: 'static,
{
    fn provide_any(&self, injector: &Injector) -> Box<dyn Any> {
        Box::new(self.p.provide(injector))
    }
}

pub trait Constructor<A, R> {
    fn new(&self, injector: &Injector) -> R;
}

pub(crate) struct ConstructorProvider<A, T, C: Constructor<A, T>> {
    pub(crate) constructor: C,
    pub(crate) pa: PhantomData<A>,
    pub(crate) pt: PhantomData<T>,
}

impl<A, T: 'static, C> ProviderAny for ConstructorProvider<A, T, C>
where
    C: Constructor<A, T>,
{
    fn provide_any(&self, injector: &Injector) -> Box<dyn Any> {
        Box::new(self.constructor.new(injector))
    }
}

impl<A, T: 'static, C> Provider for ConstructorProvider<A, T, C>
where
    C: Constructor<A, T>,
{
    type Provided = T;
    fn provide(&self, injector: &Injector) -> Self::Provided {
        self.constructor.new(injector)
    }
}

pub(crate) struct SingletonProvider<T: Clone>(pub(crate) T);

impl<T: Clone + 'static> ProviderAny for SingletonProvider<T> {
    fn provide_any(&self, _injector: &Injector) -> Box<dyn Any> {
        Box::new(self.0.clone())
    }
}

impl<T: Clone + 'static> Provider for SingletonProvider<T> {
    type Provided = T;
    fn provide(&self, _injector: &Injector) -> Self::Provided {
        self.0.clone()
    }
}

// pub fn ImplConstructor<A, T, I, C>(c: C) -> ArcProvider<A, T, C>
// where
//     C: Constructor<A, T>,
//     I: T,
// {
//     ArcProvider {
//         p: c,
//         phantom_a: PhantomData,
//         phantom_t: PhantomData,
//     }
// }

// pub struct ArcProvider<A, T, P>
// where
//     P: Constructor<A, T>,
// {
//     p: P,
//     phantom_a: PhantomData<A>,
//     phantom_t: PhantomData<T>,
// }

// impl<A, T, P> Provider for ArcProvider<A, T, P>
// where
//     P: Constructor<A, T>,
// {
//     type Provided = Arc<T>;

//     fn provide(&self, injector: &Injector) -> Self::Provided {
//         Arc::new(self.p.new(injector))
//     }
// }

// impl<A, T, P> ProviderAny for ArcProvider<A, T, P>
// where
//     P: Constructor<A, T>,
//     T: 'static,
// {
//     fn provide_any(&self, injector: &Injector) -> Box<dyn Any> {
//         Box::new(self.p.new(injector))
//     }
// }

macro_rules! cons_provider {
    (
        [$($ty:ident),*], $last:ident
    ) => {
        #[allow(non_snake_case, unused_mut,unused_variables)]
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

all_the_tuples!(cons_provider);
