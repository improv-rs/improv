use super::prelude::*;

pub struct Asker<M: Message> {
    inner: Box<dyn Ask<M>>,
}

impl<M: Message> Asker<M> {
    #[inline]
    pub(super) fn new<A: Actor>(addr: Addr<A>) -> Self
    where
        A: Receive<M>,
    {
        Self { inner: Box::new(addr) }
    }
}

impl<M: Message> Asker<M> {
    #[inline]
    pub fn ask(
        &self,
        msg: M,
    ) -> impl Future<Output = Result<<M as Message>::Response, Disconnected>>
    {
        self.inner.ask(msg)
    }
}
