extern crate futures;
extern crate futures_cpupool;
extern crate tokio_core;

use self::futures::{future, Future, Stream};
use self::future::BoxFuture;
use self::tokio_core::reactor::Core;
use self::futures_cpupool::CpuPool;
use std::{ops, io};

pub struct Delegator<T> {
    data: Option<Vec<Result<T, ()>>>,
}

impl<T> Delegator<T>
    where T: Send + Sync + ops::Add<Output = T> + 'static
{
    pub fn new(data: Vec<T>) -> io::Result<Self> {
        Ok(Delegator { data: Some(data.into_iter().map(Ok).collect()) })
    }

    pub fn ship(&mut self, initializer: T) -> BoxFuture<T, io::Error> {
        let data = self.data.take().expect("must have data to process");
        let mut core = Core::new().expect("failed to create core");
        let pool = CpuPool::new_num_cpus();

        let stream = futures::stream::iter(data)
            .and_then(|x| pool.spawn_fn(|| Ok(x)))
            .fold(initializer, |a, b| Ok(a + b))
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "failure running thread jobs"));

        future::result(core.run(stream)).boxed()
    }
}
