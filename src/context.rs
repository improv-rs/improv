use super::prelude::*;

pub struct Context<A: Actor> {
    pub(super) inbox: mpsc::UnboundedReceiver<Box<dyn Envelope<Actor = A>>>,
    pub(super) keep_running: bool,

    addr: Addr<A>,
}

impl<A: Actor> Context<A> {
    pub(super) fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        Self {
            inbox:        rx,
            addr:         Addr::new(tx),
            keep_running: true,
        }
    }
}

impl<A: Actor> Context<A> {
    #[inline]
    pub fn addr(&self) -> Addr<A> { self.addr.clone() }

    #[inline]
    pub fn notify_after<M: Message<Response = ()>>(
        &self,
        duration: Duration,
        msg_factory: impl FnOnce() -> M + Send + 'static,
    ) where
        A: Receive<M>,
    {
        let addr = self.addr();
        tokio::spawn(async move {
            delay_for(duration).await;
            let _ = addr.tell(msg_factory());
        });
    }

    #[inline]
    pub fn notify_periodic<M: Message<Response = ()>>(
        &self,
        duration: Duration,
        msg_factory: impl FnOnce() -> M + Clone + Send + 'static,
    ) where
        A: Receive<M>,
    {
        let addr = self.addr();
        tokio::spawn(async move {
            loop {
                delay_for(duration).await;
                let _ = addr.tell((msg_factory.clone())());
            }
        });
    }

    #[inline]
    pub fn stop(&mut self) { self.keep_running = false; }
}
