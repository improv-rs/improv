use super::prelude::*;

#[async_trait]
pub trait Receive<M>: Actor
where
    M: Message,
{
    async fn receive(
        &mut self,
        msg: M,
        cx: &mut Context<Self>,
    ) -> <M as Message>::Response;
}
