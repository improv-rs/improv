use super::prelude::*;

pub(super) trait NowOrNever: Future {
    fn now_or_never(self) -> Option<Self::Output>;
}

impl<T: Future> NowOrNever for T {
    fn now_or_never(self) -> Option<Self::Output> {
        let fut = self;
        pin!(fut);
        if let Poll::Ready(output) =
            fut.poll(&mut task::Context::from_waker(noop_waker_ref()))
        {
            Some(output)
        } else {
            None
        }
    }
}
