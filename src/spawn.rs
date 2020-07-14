use super::prelude::*;

pub fn spawn<A: Actor>(spawner: impl FnOnce(&mut Context<A>) -> A) -> Addr<A> {
    let mut cx = Context::new();
    let actor = spawner(&mut cx);
    let addr = cx.addr();
    let cell = Cell::new(actor, cx);
    tokio::spawn(cell.run());
    addr
}
