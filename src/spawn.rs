use super::prelude::*;

pub fn spawn<A: Actor>(
    spawner: impl FnOnce(&mut Context<A>) -> A,
) -> Address<A> {
    let mut cx = Context::new();
    let actor = spawner(&mut cx);
    let address = cx.address();
    let cell = Cell::new(actor, cx);
    tokio::spawn(cell.run());
    address
}
