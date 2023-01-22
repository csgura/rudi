mod binder;
mod binding;
mod implements;
mod injector;
mod module;
mod provider;

pub use binder::Binder;
pub use implements::Implements;
pub use injector::Injector;
pub use module::AbstractModule;
pub use provider::Provider;

// impl<A,T,C> Provider for C where
// C : Constructor<A,T> {
//     type ProvidedType = T;

//     fn provide(&self,  injector : &Injector ) -> Self::ProvidedType {
//         self(injector)
//     }
// }
