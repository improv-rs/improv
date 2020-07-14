use super::prelude::*;

#[derive(Eq, PartialEq)]
pub struct Disconnected {
    _priv: (),
}

impl Disconnected {
    pub(super) fn new() -> Self { Self { _priv: () } }
}

impl fmt::Debug for Disconnected {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Disconnected {{ .. }}")
    }
}
