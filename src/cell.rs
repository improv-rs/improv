use super::prelude::*;

pub(super) struct Cell<A: Actor> {
    actor: A,
    cx:    Context<A>,
}

const YIELD_EVERY: usize = 32;

impl<A: Actor> Cell<A> {
    pub(super) fn new(actor: A, cx: Context<A>) -> Self { Self { actor, cx } }

    pub(super) async fn run(self) {
        let Self { mut actor, mut cx } = self;

        return_if!(!cx.keep_running);
        actor.started(&mut cx);
        return_if!(!cx.keep_running);

        // Main actor event loop
        while let Some(envelope) = cx.inbox.next().await {
            envelope.handle(&mut actor, &mut cx).await;
            break_if!(!cx.keep_running);
        }

        // Deplete all pending messages unless stop is explicitly requested.
        let mut yielder = RegularYielder::new(YIELD_EVERY);
        while let Some(Some(envelope)) = cx.inbox.next().now_or_never() {
            // If there are a lot of messages in the inbox we want to avoid
            // starving other tasks waiting on the executor. By forcing a yield
            // at regular intervals we ensure that other tasks get a proper
            // chance to run.
            yielder.yield_with(envelope.handle(&mut actor, &mut cx)).await;
            return_if!(!cx.keep_running);
        }

        actor.stopped(&mut cx);
    }
}
