use super::prelude::*;

pub(super) type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;
