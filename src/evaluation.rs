use crate::{evaluator::InstanceEvaluator, Instance, Suite};
use mahf::{Configuration, State};
use std::thread;

/// Evaluate a [Suite] efficiently.
///
/// The suite will be evaluated on `threads` threads in parallel.
///
/// `on_setup` will be called to configure state during [Configuration::optimize_with].
/// It can be used to setup logging or insert custom state.
///
/// `on_complete` will be called after each evaluation.
/// It can be used to write the log to a file or display progress.
pub fn evaluate_suite(
    suite: Suite,
    configuration: Configuration<Instance>,
    threads: u32,

    on_setup: impl Fn(&mut State<Instance>) + Send + Sync,
    on_complete: impl Fn(Instance, State<Instance>) + Send + Sync,
) -> anyhow::Result<()> {
    let mut suite = suite;
    let configuration = &configuration;
    let on_setup = &on_setup;
    let on_complete = &on_complete;

    thread::scope(move |scope| {
        scope.spawn(move || {
            let mut pool = scoped_threadpool::Pool::new(threads);

            while let Some(instance) = suite.next() {
                // Create a new suite, because COCO doesn't guarantee that
                // multiple problems can be created from one suite simultaneously.
                let mut suite = suite.clone();

                pool.scoped(move |pool| {
                    pool.execute(move || {
                        let state = configuration.optimize_with(&instance, |state| {
                            state.insert(InstanceEvaluator::new(&mut suite, &instance));

                            on_setup(state);
                        });

                        on_complete(instance, state);
                    });
                });
            }
        });

        Ok(())
    })
}
