use super::prelude::*;

pub struct Probe<A: Actor> {
    inbox: mpsc::UnboundedReceiver<Box<dyn Envelope<Actor = A>>>,
    addr:  Addr<A>,
}

impl<A: Actor> Default for Probe<A> {
    fn default() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        Self { inbox: rx, addr: Addr::new(tx) }
    }
}

impl<A: Actor> Probe<A> {
    pub fn new() -> Self { Self::default() }

    pub fn addr(&self) -> Addr<A> { self.addr.clone() }

    pub async fn receive<M: Message>(&mut self) -> M
    where
        A: Receive<M>,
    {
        if let Ok(msg) = self
            .inbox
            .next()
            .await
            .expect("all senders disconnected")
            .into_any()
            .downcast::<M>()
        {
            *msg
        } else {
            panic!("received an unexpected message variant");
        }
    }
}
