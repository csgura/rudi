use std::marker::PhantomData;

use crate::Injector;

pub trait Provider<T> {
    fn provide(&self, injector: &Injector) -> T;
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

pub(crate) struct ConstructorProvider<A, T, C: Constructor<A, T>> {
    pub(crate) constructor: C,
    pub(crate) pa: PhantomData<A>,
    pub(crate) pt: PhantomData<T>,
}

impl<A, T, C> Provider<T> for ConstructorProvider<A, T, C>
where
    C: Constructor<A, T>,
{
    fn provide(&self, injector: &Injector) -> T {
        self.constructor.new(injector)
    }
}

pub(crate) struct SingletonProvider<T: Clone>(pub(crate) T);

impl<T: Clone> Provider<T> for SingletonProvider<T> {
    fn provide(&self, injector: &Injector) -> T {
        self.0.clone()
    }
}

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
