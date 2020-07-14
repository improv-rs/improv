use super::prelude::*;

pub struct Addr<A: Actor> {
    inner: Arc<mpsc::UnboundedSender<Box<dyn Envelope<Actor = A>>>>,
}

impl<A: Actor> Addr<A> {
    #[inline]
    pub(super) fn new(
        tx: mpsc::UnboundedSender<Box<dyn Envelope<Actor = A>>>,
    ) -> Self {
        Self { inner: Arc::new(tx) }
    }
}

impl<A: Actor> Addr<A> {
    pub fn tell<M: Message<Response = ()>>(
        &self,
        msg: M,
    ) -> Result<(), Disconnected>
    where
        A: Receive<M>,
    {
        let teller = TellEnvelope::new(msg);
        self.inner.send(Box::new(teller)).map_err(|_| Disconnected::new())
    }

    pub fn ask<M: Message>(
        &self,
        msg: M,
    ) -> impl Future<Output = Result<<M as Message>::Response, Disconnected>> + 'static
    where
        A: Receive<M>,
    {
        let (asker, rx) = AskEnvelope::new(msg);
        let inner = Arc::clone(&self.inner);
        async move {
            inner.send(Box::new(asker)).map_err(|_| Disconnected::new())?;
            rx.await
        }
    }

    #[inline]
    pub fn into_teller<M: Message<Response = ()>>(self) -> Teller<M>
    where
        A: Receive<M>,
    {
        Teller::new(self)
    }

    #[inline]
    #[inline]
    pub fn into_asker<M: Message>(self) -> Asker<M>
    where
        A: Receive<M>,
    {
        Asker::new(self)
    }
}

impl<A: Actor, M: Message<Response = ()>> Tell<M> for Addr<A>
where
    A: Receive<M>,
{
    fn tell(&self, msg: M) -> Result<(), Disconnected> { self.tell(msg) }
}

impl<A: Actor, M: Message> Ask<M> for Addr<A>
where
    A: Receive<M>,
{
    fn ask(
        &self,
        msg: M,
    ) -> BoxFuture<'static, Result<<M as Message>::Response, Disconnected>>
    {
        self.ask(msg).boxed()
    }
}

impl<A: Actor> fmt::Debug for Addr<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Address({:p})", self.inner)
    }
}

impl<A: Actor> Clone for Addr<A> {
    fn clone(&self) -> Self { Self { inner: Arc::clone(&self.inner) } }
}
