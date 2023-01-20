use std::{sync::{Arc, Mutex}, collections::HashMap, any::{Any, TypeId}, marker::PhantomData};



pub trait Provider {
    type ProvidedType;

    fn provide(&self,  injector : &Injector ) -> Self::ProvidedType;
}

#[derive(Clone)]
pub struct Injector {
    binds : Arc<Mutex<HashMap<String,Box<dyn Any>>>> ,
    instances : Arc<Mutex<HashMap<String,Box<dyn Any>>>> ,

}

trait Singleton {
    type ProvidedType;
    fn get(&self) -> Arc<Self::ProvidedType>;

}

#[derive(Clone)]
pub struct Binder {
    binds : Arc<Mutex<HashMap<TypeId,BoxedProvider>>> ,
}


impl Binder {
    pub fn bind<T>(&self ) -> BindTo<T> where T: 'static  {
        BindTo {  binder : self.clone(), typeId: TypeId::of::<T>(), phantom: PhantomData }
    }
}

impl<T,F> Provider for F
where
    F: Fn(&Injector) -> T,
{
    type ProvidedType = T;
    fn provide(&self,  injector : &Injector ) -> Self::ProvidedType {
        self(injector)
    }

}

pub trait Constructor<A,R> {
    fn new(&self, injector : &Injector) -> R;
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
        where F : Fn($(Arc<$ty>,)*) -> $last,
        $(
            $ty : 'static, 
        )*
         {
            fn new(&self, injector : &Injector) -> $last {
                $(
                    let $ty = injector.get_instance::<$ty>().unwrap().clone();
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
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13], T14);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14], T15);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15], T16);
    };
}

all_the_tuples!(impl_handler);



struct BoxedProvider(Box<dyn Any>);

pub struct BindTo<T : ?Sized> {
    binder : Binder,
    typeId : TypeId,
    phantom : PhantomData<T>,
}

impl<T:?Sized> BindTo<T> {
    pub fn to_provider<F>( & self,  f : F) where T : 'static, F : Fn(&Injector) -> T + 'static {

        let prov : BoxedProvider = BoxedProvider(Box::new(f));

        let mut m = self.binder.binds.lock().unwrap();
        
        m.insert(self.typeId, prov);

    }

    pub fn to_constructor<A,C>(&self , c : C) where C : Constructor<A,T> + 'static, T : Sized + 'static {
       self.to_provider(move |i| c.new(i))
    } 
}

pub trait AbstractModule {
    fn config( binder : &mut Binder );
}

impl Injector {
    // pub fn get_instance<T>( &self, name : &str ) -> Option<Arc<T>> where T : 'static{

    //     let m = self.instances.lock().unwrap();

    //     let ret = m.get(name);
    //     let ret = ret.and_then(|x| {
    //          x.downcast_ref::<Box<dyn Singleton<ProvidedType = T>>>()
    //     }).map(|x|x.get());

    //     ret

    // } 

    pub fn get_instance<T>( &self) -> Option<Arc<T>> where T : 'static{
        todo!();
    }
}
