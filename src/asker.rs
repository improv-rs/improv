use super::prelude::*;

pub struct Asker<M: Message> {
    inner: Box<dyn Ask<M>>,
}

impl<M: Message> Asker<M> {
    #[inline]
    pub(super) fn new<A: Actor>(address: Address<A>) -> Self
    where
        A: Receive<M>,
    {
        Self { inner: Box::new(address) }
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
