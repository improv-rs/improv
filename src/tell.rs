use super::prelude::*;

pub(super) trait Tell<M: Message<Response = ()>> {
    fn tell(&self, msg: M) -> Result<(), Disconnected>;
}
