extern crate futures;
extern crate tokio_core;
extern crate futures_cpupool;

use self::futures::future::{Future, BoxFuture};
use self::tokio_core::reactor::Core;
use self::futures_cpupool::CpuPool;
use std::io;

pub struct Delegator<T>
{
    pub pool: CpuPool,
    pub data: Option<T>,
}

impl<T> Delegator<T>
    where T: Send + Clone
{
    pub fn new() -> io::Result<Self> {
        Ok(Delegator {
            pool: CpuPool::new_num_cpus(),
            data: None,
        })
    }
}

pub trait Allocate {
    type Data: Send + Clone;
    type Item;
    type Error;
    fn ship(&self, data: &Self::Data) -> BoxFuture<Self::Item, Self::Error>;
}
