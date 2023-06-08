use crate::{evaluator::InstanceEvaluator, Instance, Suite};
use mahf::{Configuration, State};
use std::thread;

pub fn evaluate_suite(
    suite: Suite,
    configuration: Configuration<Instance>,
    threads: u32,

    on_setup: impl Fn(&mut State<Instance>) + Send + Sync,
    on_complete: impl Fn(State<Instance>) + Send + Sync,
) -> anyhow::Result<()> {
    let mut suite = suite;
    let configuration = &configuration;
    let on_setup = &on_setup;
    let on_complete = &on_complete;

    thread::scope(move |scope| {
        scope.spawn(move || {
            let mut pool = scoped_threadpool::Pool::new(threads);

            while let Some(instance) = suite.next_instance() {
                // Create a new suite, because COCO doesn't guarantee that
                // multiple problems can be created from one suite simultaneously.
                let mut suite = suite.clone();

                pool.scoped(move |pool| {
                    pool.execute(move || {
                        let state = configuration.optimize_with(&instance, |state| {
                            state.insert(InstanceEvaluator::new(&mut suite, &instance));

                            on_setup(state);
                        });

                        on_complete(state);
                    });
                });
            }
        });

        Ok(())
    })
}
