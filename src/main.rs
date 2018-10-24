extern crate ks2_timelapse;
extern crate tokio;
extern crate futures;

use futures::{
    Future,
    Stream,
};

use std::error::Error;

fn main() -> Result<(), Box<Error>> {

    let runtime = tokio::runtime::Runtime::new()?;

    let photo_stream = ks2_timelapse::PhotoStream::new();

    runtime.spawn(photo_stream);

    runtime.shutdown_on_idle().wait().unwrap();
    Ok(())
}


