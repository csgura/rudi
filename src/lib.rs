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
pub use provider::Constructor;
// pub use provider::std::sync::ArcProvider;
// pub use provider::ImplConstructor;
pub use provider::InterceptFunc;
pub use provider::Provider;
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
        $e.bind::<std::sync::Arc<dyn $ty>>()
    };
}

#[macro_export]
macro_rules! intercept_dyn {
    ($e:expr, $ty:tt) => {
        $e.intercept::<std::sync::Arc<dyn $ty>>()
    };
}

#[macro_export]
macro_rules! bind_dyn_constructor {
    ($e:expr, $ty:tt, $cons:tt) => {
        $e.bind::<std::sync::Arc<dyn $ty>>()
            .to_provider($crate::ProviderFunc(|i| {
                let ret: std::sync::Arc<dyn $ty> = std::sync::Arc::new(i.inject_and_call($cons));
                ret
            }))
    };

    ($e:expr, $ty:tt, $p:path) => {
        $e.bind::<std::sync::Arc<dyn $ty>>()
            .to_provider($crate::ProviderFunc(|i| {
                let ret: std::sync::Arc<dyn $ty> = std::sync::Arc::new(i.inject_and_call($p));
                ret
            }))
    };
}

#[macro_export]
macro_rules! get_instance {
    ($e:expr, Option<$ty:ty>) => {{
        $e.get_instance::<$ty>()
    }};
    ($e:expr, $ty:ty) => {{
        $e.get_instance::<$ty>()
    }};
}

#[macro_export]
macro_rules! get_instance_dyn {
    ($e:expr, $ty:tt) => {
        $e.get_instance::<std::sync::Arc<dyn $ty>>()
    };
}

#[macro_export]
macro_rules! new_injector {
    ($e:expr, $($name:expr),*) => {
        $e.new_injector(vec![$(String::from($name),)*])
    };

    ($e:expr) => {
        $e.new_injector(vec![])
    };
}
