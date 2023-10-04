use mahf_coco::StandardEvaluator;

#[test]
fn can_create_instances() {
    let mut suite = mahf_coco::Suite::new(mahf_coco::Name::Bbob);

    while let Some(instance) = suite.next() {
        let _evaluator = StandardEvaluator::new(&mut suite, &instance);
    }
}
