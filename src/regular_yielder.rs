use super::prelude::*;

pub(super) struct RegularYielder {
    yield_every: usize,
    counter:     usize,
}

impl RegularYielder {
    pub(super) fn new(yield_every: usize) -> Self {
        assert_ne!(yield_every, 0);
        Self { yield_every, counter: 0 }
    }

    pub(super) fn yield_with<'a, F: Future + 'a>(
        &'a mut self,
        fut: F,
    ) -> impl Future<Output = <F as Future>::Output> + 'a {
        pin_project! {
            struct YielderImpl<'a, F: Future> {
                #[pin]
                fut:    F,
                polled: bool,
                cache:  Option<<F as Future>::Output>,
                parent: &'a mut RegularYielder,
            }
        }

        impl<'a, F: Future + 'a> Future for YielderImpl<'a, F> {
            type Output = <F as Future>::Output;

            fn poll(
                self: Pin<&mut Self>,
                cx: &mut task::Context<'_>,
            ) -> Poll<Self::Output> {
                let this = self.project();

                // Previously polled output cached after a co-op yield.
                if let Some(output) = this.cache.take() {
                    return Poll::Ready(output);
                }

                match this.fut.poll(cx) {
                    // Already polled - yield has previously occurred
                    Poll::Ready(output) if *this.polled => Poll::Ready(output),

                    // First poll - current counter has exceeded the limit. We
                    // need to force a yield to the executor.
                    // The result of the poll is cached and immediately returned
                    // on the next call to poll.
                    Poll::Ready(output)
                        if this.parent.counter >= this.parent.yield_every =>
                    {
                        *this.polled = true;
                        this.parent.counter = 0;
                        *this.cache = Some(output);
                        cx.waker().wake_by_ref();
                        Poll::Pending
                    }

                    // First poll - still within yield budget.
                    Poll::Ready(output) => {
                        this.parent.counter += 1;
                        Poll::Ready(output)
                    }

                    // First poll - counter doesn't need increment as we are
                    // already yielding.
                    Poll::Pending => {
                        *this.polled = true;
                        Poll::Pending
                    }
                }
            }
        }

        YielderImpl { fut, polled: false, cache: None, parent: self }
    }
}
