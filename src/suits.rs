use crate::Instance;
use coco_rs::SuiteName;

#[derive(Clone)]
pub struct Suite {
    inner: coco_rs::Suite,
}

impl Suite {
    pub fn new(name: SuiteName) -> Self {
        Suite {
            inner: coco_rs::Suite::new(name, "", "").unwrap(),
        }
    }

    pub fn with_instances(name: SuiteName, instances: &str) -> Option<Self> {
        Some(Suite {
            inner: coco_rs::Suite::new(name, instances, "")?,
        })
    }

    pub fn next_instance(&mut self) -> Option<Instance> {
        Some(Instance::from(&self.inner.next_problem(None)?))
    }

    pub fn problem_for_instance(&mut self, instance: &Instance) -> coco_rs::Problem {
        self.inner
            .problem_by_function_dimension_instance(
                instance.function_idx,
                instance.dimension_idx,
                instance.instance_idx,
            )
            .unwrap()
    }
}
