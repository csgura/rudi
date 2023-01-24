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
pub use module::BindFunc;
pub use module::CombinedModule;
pub use module::OverridableModule;
pub use module::OverridedModule;
// pub use provider::ArcProvider;
// pub use provider::ImplConstructor;
pub use provider::ProviderAny;
pub use provider::ProviderFunc;

#[macro_export]
macro_rules! bind {
    ($e:expr, $ty:ty) => {
        $e.bind::<$ty>()
    };
}

#[macro_export]
macro_rules! bind_dyn {
    ($e:expr, $ty:tt) => {
        $e.bind::<Arc<dyn $ty>>()
    };
}

// #[macro_export]
// macro_rules! bind_dyn_constructor {
//     ($e:expr, $ty:tt, $cons:tt) => {
//         $e.bind::<Arc<dyn $ty>>()
//             .to_provider($crate::ProviderFunc(|i| Arc::new(i.inject_and_call($cons))))
//     };
// }
