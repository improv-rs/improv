use super::prelude::*;

pub(super) trait Boxed: Future {
    fn boxed(self) -> BoxFuture<'static, Self::Output>;
}

impl<T: Future + Send + 'static> Boxed for T {
    fn boxed(self) -> BoxFuture<'static, Self::Output> { Box::pin(self) }
}
