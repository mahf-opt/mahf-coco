use crate::Instance;
use coco_rs::SuiteName;

/// A COCO benchmark suite.
///
/// It's a thin wrapper around [coco_rs::Suite] to simplify integration with MAHF.
#[derive(Clone)]
pub struct Suite {
    inner: coco_rs::Suite,
}

impl Suite {
    /// Creates a new suite with the given name.
    pub fn new(name: SuiteName) -> Self {
        Suite {
            inner: coco_rs::Suite::new(name, "", "").unwrap(),
        }
    }

    /// Create a new suite with the given name and instances.
    ///
    /// Fails if the provided instances don't exist.
    pub fn with_instances(name: SuiteName, instances: &str) -> Option<Self> {
        Some(Suite {
            inner: coco_rs::Suite::new(name, instances, "")?,
        })
    }

    /// Returns the total number of problems in the suite.
    pub fn number_of_problems(&self) -> usize {
        self.inner.number_of_problems()
    }

    /// Returns the [coco_rs::Problem] for a specific instance.
    pub(crate) fn problem_for_instance(&mut self, instance: &Instance) -> coco_rs::Problem {
        self.inner
            .problem_by_function_dimension_instance(
                instance.function_idx,
                instance.dimension_idx,
                instance.instance_idx,
            )
            .unwrap()
    }
}

impl Iterator for Suite {
    type Item = Instance;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Instance::from(&self.inner.next_problem(None)?))
    }
}
