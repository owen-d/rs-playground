extern crate futures;
extern crate futures_cpupool;

use futures::{future, Future, Stream};
use future::BoxFuture;
use futures::sync::mpsc;
use futures_cpupool::{CpuPool, CpuFuture};
use std::io;

pub struct Delegator<T> {
    pub pool: CpuPool,
    pub data: Vec<Result<T, ()>>,
}

impl<T> Delegator<T>
    where T: Send + Sync + std::ops::Add<Output=T>
{
    pub fn new(data: Vec<T>) -> io::Result<Self> {
        Ok(Delegator {
            pool: CpuPool::new_num_cpus(),
            data: data.into_iter().map(Ok).collect(),
        })
    }

    fn ship(&self, initializer: T) -> BoxFuture<T, io::Error> {
        let mut stream = futures::stream::iter(self.data).map(|x| {
            self.pool.spawn_fn(|| {
                let res: Result<T, ()> = Ok(x);
                res
            })
        });

        let res = stream.and_then(|x| x.wait()).collect().wait().map(|completed| {
            completed.into_iter().fold(initializer, |a, b| a + b)
        }).map_err(|e| io::Error::new(io::ErrorKind::Other, "failure running thread jobs"));
        return future::result(res).boxed()
    }
}
