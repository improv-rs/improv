use super::prelude::*;

pub(super) fn noop_waker_ref() -> &'static Waker {
    const VTABLE: RawWakerVTable = RawWakerVTable::new(clone, noop, noop, noop);

    unsafe fn clone(_: *const ()) -> RawWaker { raw_waker() }
    unsafe fn noop(_: *const ()) {}

    fn raw_waker() -> RawWaker { RawWaker::new(ptr::null(), &VTABLE) }
    fn waker() -> Waker { unsafe { Waker::from_raw(raw_waker()) } }

    thread_local!(static WAKER: UnsafeCell<Waker> = UnsafeCell::new(waker()));
    WAKER.with(|l| unsafe { &*l.get() })
}
