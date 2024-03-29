cfg_rt! {
    use crate::future::Future;
    use crate::runtime::basic_scheduler;
    use crate::task::JoinHandle;
}

#[derive(Debug, Clone)]
pub(crate) enum Spawner {
    #[cfg(feature = "rt")]
    Basic(basic_scheduler::Spawner),
}

impl Spawner {
    pub(crate) fn shutdown(&mut self) {}
}

cfg_rt! {
    impl Spawner {
        pub(crate) fn spawn<F>(&self, future: F) -> JoinHandle<F::Output>
        where
            F: Future + Send + 'static,
            F::Output: Send + 'static,
        {
            match self {
                #[cfg(feature = "rt")]
                Spawner::Basic(spawner) => spawner.spawn(future),
            }
        }
    }
}
