extern crate futures;
extern crate futures_cpupool;

use self::futures::{future, Future, Stream};
use self::future::BoxFuture;
// use futures::sync::mpsc;
use self::futures_cpupool::CpuPool;
use std::io;
use std::ops;

pub struct Delegator<T> {
    pub pool: CpuPool,
    pub data: Option<Vec<Result<T, ()>>>,
}

impl<T> Delegator<T>
    where T: Send + Sync + ops::Add<Output = T> + 'static
{
    pub fn new(data: Vec<T>) -> io::Result<Self> {
        Ok(Delegator {
            pool: CpuPool::new_num_cpus(),
            data: Some(data.into_iter().map(Ok).collect()),
        })
    }

    pub fn ship(&mut self, initializer: T) -> BoxFuture<T, io::Error> {
        let data = self.data.take().expect("must have data to process");
        let stream = futures::stream::iter(data).map(|x| {
            self.pool.spawn_fn(move || {
                let res: Result<T, ()> = Ok(x);
                res
            })
        });

        let res = stream.and_then(|x| x.wait())
            .collect()
            .wait()
            .map(|completed| completed.into_iter().fold(initializer, |a, b| a + b))
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "failure running thread jobs"));
        return future::result(res).boxed();
    }
}
