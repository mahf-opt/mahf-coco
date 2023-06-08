//! This crate provides a [MAHF](https://github.com/mahf-opt/mahf) problem instance for [COCO](https://github.com/numbbo/coco).
//!
//! To use this, create a [Suite] and call [evaluate_suite] with it.

mod evaluation;
mod evaluator;
mod instance;
mod suits;

pub use coco_rs::SuiteName;

pub use evaluation::evaluate_suite;
pub use evaluator::InstanceEvaluator;
pub use instance::Instance;
pub use suits::Suite;
