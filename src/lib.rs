//! [MAHF](https://github.com/mahf-opt/mahf) bindings for the [COCO](https://github.com/numbbo/coco) benchmarking framework.
//!
//! To use this, create a [`Suite`] and call [`evaluate_suite`] with it.

// Execute doc tests for external files.
#[cfg(doctest)]
mod test_readme {
    macro_rules! external_doc_test {
        ($x:expr) => {
            #[doc = $x]
            extern "C" {}
        };
    }

    external_doc_test!(include_str!("../README.md"));
}

pub use coco_rs::{suite::Name, LogLevel};
pub use coco_accelerated::Context;

pub mod backends {
    pub use coco_accelerated::backends::Backend;
    #[cfg(feature = "cuda")]
    pub use coco_accelerated::backends::Cuda;
    #[cfg(feature = "multicore")]
    pub use coco_accelerated::backends::MultiCore;
    #[cfg(feature = "opencl")]
    pub use coco_accelerated::backends::OpenCl;
    #[cfg(feature = "c")]
    pub use coco_accelerated::backends::C;
}

mod evaluation;
mod evaluator;
mod instance;
mod suits;

pub use evaluation::evaluate_suite;
pub use evaluator::{AcceleratedEvaluator, StandardEvaluator, ThreadSafeEvaluator};
pub use instance::Instance;
pub use suits::{Instances, Options, Suite};
