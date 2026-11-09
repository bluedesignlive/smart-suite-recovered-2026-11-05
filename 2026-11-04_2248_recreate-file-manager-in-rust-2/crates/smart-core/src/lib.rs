//! Smart Files core — everything except pixels. Frontends (egui today; gpui, native toolkits tomorrow)
//! drive this crate through [`store::Store`] and `Action`s, and render what it exposes.
pub mod ai;
pub mod archive;
pub mod demo;
pub mod engine;
pub mod model;
pub mod search;
pub mod store;
pub mod sys;
pub mod treemap;
pub mod vfs;

pub use engine::*;
pub use model::*;
pub use store::*;
pub use ai::{AiMsg, CleanCat, CleanItem, CleanReport, LensItem, Plan, PlanOp};
