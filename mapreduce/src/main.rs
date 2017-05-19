mod lib;
extern crate futures;

use std::io;
use futures::future::{Future, BoxFuture};
use futures::future;


impl<T> lib::Allocate for lib::Delegator<T>
    where T: Send + Clone
{
    type Data = T;
    type Item = u32;
    type Error = io::Error;
    fn ship(&self, data: &Self::Data) -> BoxFuture<Self::Item, Self::Error> {
        let res = self.pool.spawn_fn(|| {
            Ok(1)
        }).map(|x| future::ok(x));

        Box::new(res)
    }
}

fn main() {}
