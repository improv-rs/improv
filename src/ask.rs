use super::prelude::*;

pub(super) trait Ask<M: Message> {
    fn ask(
        &self,
        msg: M,
    ) -> BoxFuture<'static, Result<<M as Message>::Response, Disconnected>>;
}
