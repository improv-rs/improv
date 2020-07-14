use super::prelude::*;

pub(super) struct Cell<A: Actor> {
    actor: A,
    cx:    Context<A>,
}

const YIELD_EVERY: usize = 32;

impl<A: Actor> Cell<A> {
    pub(super) fn new(actor: A, cx: Context<A>) -> Self { Self { actor, cx } }

    pub(super) async fn run(mut self) {
        self.run_impl().await;
        let Self { actor, cx } = &mut self;
        actor.stopped(cx);
    }

    async fn run_impl(&mut self) {
        let Self { actor, cx } = self;

        return_if!(!cx.keep_running);
        actor.started(cx);
        return_if!(!cx.keep_running);

        // Main actor event loop
        while let Some(envelope) = cx.inbox.next().await {
            envelope.handle(actor, cx).await;
            break_if!(!cx.keep_running);
        }

        // Deplete all pending messages unless stop is explicitly requested.
        let mut yielder = RegularYielder::new(YIELD_EVERY);
        while let Some(Some(envelope)) = cx.inbox.next().now_or_never() {
            // If there are a lot of messages in the inbox we want to avoid
            // starving other tasks waiting on the executor. By forcing a yield
            // at regular intervals we ensure that other tasks get a proper
            // chance to run.
            yielder.yield_with(envelope.handle(actor, cx)).await;
            break_if!(!cx.keep_running);
        }
    }
}
