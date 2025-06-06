//! # Floem Reactive
//!
//! [`RwSignal::new_split`](RwSignal::new_split) returns a separated [`ReadSignal`] and [`WriteSignal`] for a variable.
//! An existing `RwSignal` may be converted using [`RwSignal::read_only`](RwSignal::read_only)
//! and [`RwSignal::write_only`](RwSignal::write_only) where necessary, but the reverse is not possible.

mod base;
mod context;
mod derived;
mod effect;
mod id;
mod impls;
mod memo;
mod read;
mod runtime;
mod scope;
mod signal;
mod trigger;
mod write;

pub use base::{create_base_signal, BaseSignal};
pub use context::{provide_context, use_context};
pub use derived::{create_derived_rw_signal, DerivedRwSignal};
pub use effect::{
    batch, create_effect, create_stateful_updater, create_tracker, create_updater, untrack,
    SignalTracker,
};
pub use memo::{create_memo, Memo};
pub use read::{ReadSignalValue, SignalGet, SignalRead, SignalTrack, SignalWith};
pub use scope::{as_child_of_current_scope, with_scope, Scope};
pub use signal::{create_rw_signal, create_signal, ReadSignal, RwSignal, WriteSignal};
pub use trigger::{create_trigger, Trigger};
pub use write::{SignalUpdate, SignalWrite, WriteSignalValue};
