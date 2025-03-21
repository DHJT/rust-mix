use std::any::Any;
use std::sync::Arc;

pub use self::time::*;
pub use self::file::*;

pub mod time;
pub mod file;
pub mod custom_error;
#[allow(non_snake_case)]
mod CustomError;
pub mod redis_util;
mod sqlite_util;
mod sqlite_tool_async;

pub fn is_blank(path: &str) -> bool {
    path.trim().is_empty()
}

/// not a general implememtation,
/// only used in our `core::flow::WarmUpCalculator`,
/// which won't overflow as long as parameter in rule is rational
#[allow(unused)]
pub(crate) fn next_after(x: f64) -> f64 {
    let x = x.to_bits();
    let x = if (x >> 63) == 0 { x + 1 } else { x - 1 };
    f64::from_bits(x)
}

/// Trait for upcast/downcast
pub trait AsAny: Any + Send + Sync {
    fn as_any(&self) -> &(dyn Any + Send + Sync);
    fn as_any_arc(self: Arc<Self>) -> Arc<dyn Any + Send + Sync>;
}

// impl the required AsAny trait for structs
impl<T: Any + Send + Sync> AsAny for T {
    fn as_any(&self) -> &(dyn Any + Send + Sync) {
        self
    }

    fn as_any_arc(self: Arc<Self>) -> Arc<dyn Any + Send + Sync> {
        self
    }
}
