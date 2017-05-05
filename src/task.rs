use std::marker::PhantomData;

use {ResourceId, Resources};

/// A wrapper around a closure
/// to allow passing closures as
/// a task using `.into()`.
pub struct Closure<F, D> {
    inner: F,
    phantom: PhantomData<*const D>,
}

impl<'a, F, D> From<F> for Closure<F, D>
    where F: FnMut(D),
          D: TaskData<'a>
{
    fn from(f: F) -> Self {
        Closure {
            inner: f,
            phantom: PhantomData,
        }
    }
}

impl<'a, F, D> Task for Closure<F, D>
    where F: FnMut(D),
          D: TaskData<'a>
{
    type TaskData = D;

    fn work(&mut self, bundle: Self::TaskData) {
        (self.inner)(bundle);
    }
}

/// A `Task`, executed with a
/// set of required [`Resource`]s.
///
/// [`Resource`]: trait.Resource.html
pub trait Task {
    /// The resource bundle required
    /// to execute this task.
    ///
    /// To create such a resource bundle,
    /// simple derive `TaskData` for it.
    type TaskData;

    /// Executes the task with the required task
    /// data.
    fn work(&mut self, bundle: Self::TaskData);
}

/// A struct implementing
/// task data indicates that it
/// bundles some resources which are
/// required for the execution.
pub trait TaskData<'a> {
    /// Creates a new resource bundle
    /// by fetching the required resources
    /// from the [`Resources`] struct.
    ///
    /// # Contract
    ///
    /// Only fetch the resources you
    /// returned from `reads` / `writes`!
    ///
    /// [`Resources`]: trait.Resources.html
    fn fetch(res: &'a Resources) -> Self;

    /// A list of [`ResourceId`]s the bundle
    /// needs read access to in order to
    /// build the target resource bundle.
    ///
    /// # Contract
    ///
    /// Exactly return the dependencies you're
    /// going to `fetch`! Doing otherwise *will*
    /// cause a data race.
    ///
    /// This method is only executed once,
    /// thus the returned value may never change
    /// (otherwise it has no effect).
    unsafe fn reads() -> Vec<ResourceId>;

    /// A list of [`ResourceId`]s the bundle
    /// needs write access to in order to
    /// build the target resource bundle.
    ///
    /// # Contract
    ///
    /// Exactly return the dependencies you're
    /// going to `fetch`! Doing otherwise *will*
    /// cause a data race.
    ///
    /// This method is only executed once,
    /// thus the returned value may never change
    /// (otherwise it has no effect).
    unsafe fn writes() -> Vec<ResourceId>;
}
