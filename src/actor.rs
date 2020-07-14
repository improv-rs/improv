use super::prelude::*;

pub trait Actor: Send + Sized + 'static {
    fn started(&mut self, _cx: &mut Context<Self>) {}

    fn stopped(&mut self, _cx: &mut Context<Self>) {}
}
