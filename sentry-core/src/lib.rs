//! <p style="margin: -10px 0 0 15px; padding: 0; float: right;">
//!   <a href="https://sentry.io/"><img
//!     src="https://sentry-brand.storage.googleapis.com/sentry-logo-black.png"
//!     style="width: 260px"></a>
//! </p>
//!
//! This crate provides support for logging events and errors to the
//! [Sentry](https://sentry.io/) error logging service.
//! It represents the core of sentry and provides APIs for instrumenting code,
//! and to write integrations that can generate events from other

mod api;
mod breadcrumbs;
mod client;
mod hub;
mod scope;
mod stack;

pub use api::*;
pub use client::Client;
pub use hub::Hub;
pub use scope::Scope;
pub(crate) use stack::Stack;

pub type EventProcessor = Box<dyn Fn(Event<'static>) -> Option<Event<'static>> + Send + Sync>;

pub use breadcrumbs::IntoBreadcrumbs;
pub use sentry_types::protocol::v7 as protocol;
pub use sentry_types::protocol::v7::{Breadcrumb, Event, Level, User};

// TODO: re-evaluate what to do with these
pub mod internals {
    pub use crate::stack::ScopeGuard;
    pub use sentry_types::{
        Auth, ChronoParseError, DateTime, ParseDebugIdError, ParseProjectIdError, ProjectId,
        TimeZone, Utc, UuidVariant, UuidVersion,
    };
    pub use sentry_types::{DebugId, Dsn, ParseDsnError, Scheme, Uuid};
}