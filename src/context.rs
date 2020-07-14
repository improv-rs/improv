use super::prelude::*;

pub struct Context<A: Actor> {
    pub(super) inbox: mpsc::UnboundedReceiver<Box<dyn Envelope<Actor = A>>>,
    pub(super) keep_running: bool,

    address: Address<A>,
}

impl<A: Actor> Context<A> {
    pub(super) fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        Self {
            inbox:        rx,
            address:      Address::new(tx),
            keep_running: true,
        }
    }
}

impl<A: Actor> Context<A> {
    #[inline]
    pub fn address(&self) -> Address<A> { self.address.clone() }

    #[inline]
    pub fn notify_later<M: Message<Response = ()>>(
        &self,
        duration: Duration,
        msg_factory: impl FnOnce() -> M + Send + 'static,
    ) where
        A: Receive<M>,
    {
        let address = self.address();
        tokio::spawn(async move {
            delay_for(duration).await;
            let _ = address.tell(msg_factory());
        });
    }

    #[inline]
    pub fn notify_interval<M: Message<Response = ()>>(
        &self,
        duration: Duration,
        msg_factory: impl FnOnce() -> M + Clone + Send + 'static,
    ) where
        A: Receive<M>,
    {
        let address = self.address();
        tokio::spawn(async move {
            loop {
                delay_for(duration).await;
                let _ = address.tell((msg_factory.clone())());
            }
        });
    }

    #[inline]
    pub fn stop(&mut self) { self.keep_running = false; }
}
