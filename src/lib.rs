//! Micro actors in Rust.

#![cfg_attr(docsrs, feature(doc_cfg))]

mod prelude;

#[macro_use]
mod macros;

mod actor;
mod address;
mod ask;
mod ask_envelope;
mod asker;
mod box_future;
mod boxed;
mod cell;
mod context;
mod disconnected;
mod envelope;
mod message;
mod noop_waker_ref;
mod now_or_never;
mod receive;
mod regular_yielder;
mod spawn;
mod tell;
mod tell_envelope;
mod teller;

use ask::Ask;
use ask_envelope::AskEnvelope;
use box_future::BoxFuture;
use boxed::Boxed;
use cell::Cell;
use envelope::Envelope;
use noop_waker_ref::noop_waker_ref;
use now_or_never::NowOrNever;
use regular_yielder::RegularYielder;
use tell::Tell;
use tell_envelope::TellEnvelope;

pub use actor::Actor;
pub use address::Address;
pub use asker::Asker;
pub use context::Context;
pub use disconnected::Disconnected;
pub use message::Message;
pub use receive::Receive;
pub use spawn::spawn;
pub use teller::Teller;

#[cfg(feature = "test-util")]
mod probe;

#[cfg(feature = "test-util")]
#[cfg_attr(docsrs, doc(cfg(feature = "test-util")))]
pub use probe::Probe;

#[doc(no_inline)]
pub use async_trait::async_trait;
