use super::prelude::*;

pub(super) struct TellEnvelope<A, M> {
    msg:     M,
    _marker: PhantomData<A>,
}

unsafe impl<A, M: Send> Send for TellEnvelope<A, M> {}

impl<A, M> TellEnvelope<A, M> {
    #[inline]
    pub(super) fn new(msg: M) -> Self { Self { msg, _marker: PhantomData } }
}

#[async_trait]
impl<A: Actor, M: Message<Response = ()>> Envelope for TellEnvelope<A, M>
where
    A: Receive<M>,
{
    type Actor = A;

    async fn handle(
        self: Box<Self>,
        actor: &mut Self::Actor,
        cx: &mut Context<Self::Actor>,
    ) {
        actor.receive(self.msg, cx).await;
    }

    #[cfg(feature = "test-util")]
    fn into_any(self: Box<Self>) -> Box<dyn Any> { Box::new(self.msg) }
}
