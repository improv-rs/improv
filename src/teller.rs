use super::prelude::*;

pub struct Teller<M: Message<Response = ()>> {
    inner: Box<dyn Tell<M>>,
}

impl<M: Message<Response = ()>> Teller<M> {
    #[inline]
    pub(super) fn new<A: Actor>(addr: Addr<A>) -> Self
    where
        A: Receive<M>,
    {
        Self { inner: Box::new(addr) }
    }
}

impl<M: Message<Response = ()>> Teller<M> {
    #[inline]
    pub fn tell(&self, msg: M) -> Result<(), Disconnected> {
        self.inner.tell(msg)
    }
}
