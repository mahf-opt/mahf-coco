use mahf_coco::InstanceEvaluator;

#[test]
fn can_create_instances() {
    let mut suite = mahf_coco::Suite::new(mahf_coco::SuiteName::Bbob);

    while let Some(instance) = suite.next() {
        let _evaluator = InstanceEvaluator::new(&mut suite, &instance);
    }
}
