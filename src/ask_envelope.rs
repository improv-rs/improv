use super::prelude::*;

pub(super) struct AskEnvelope<A: Actor, M: Message> {
    msg:     M,
    resp:    oneshot::Sender<<M as Message>::Response>,
    _marker: PhantomData<A>,
}

unsafe impl<A: Actor, M: Message> Send for AskEnvelope<A, M> {}

impl<A: Actor, M: Message> AskEnvelope<A, M> {
    #[inline]
    pub(super) fn new(
        msg: M,
    ) -> (
        Self,
        impl Future<Output = Result<<M as Message>::Response, Disconnected>>,
    ) {
        let (tx, rx) = oneshot::channel();
        let rx = async move { rx.await.map_err(|_| Disconnected::new()) };
        (Self { msg, resp: tx, _marker: PhantomData }, rx)
    }
}

#[async_trait]
impl<A: Actor, M: Message> Envelope for AskEnvelope<A, M>
where
    A: Receive<M>,
{
    type Actor = A;

    async fn handle(
        self: Box<Self>,
        actor: &mut Self::Actor,
        cx: &mut Context<Self::Actor>,
    ) {
        let _ = self.resp.send(actor.receive(self.msg, cx).await);
    }

    #[cfg(feature = "test-util")]
    fn into_any(self: Box<Self>) -> Box<dyn Any> { Box::new(self.msg) }
}
