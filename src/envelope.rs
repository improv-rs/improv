use super::prelude::*;

#[async_trait]
pub trait Envelope: Send {
    type Actor: Actor;

    async fn handle(
        self: Box<Self>,
        actor: &mut Self::Actor,
        cx: &mut Context<Self::Actor>,
    );

    #[cfg(feature = "test-util")]
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}
