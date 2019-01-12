extern crate ks2_timelapse;
extern crate tokio;
extern crate futures;

use tokio::timer::Interval;
use std::time::Duration;

use futures::{
    Future,
    Stream,
};

use std::error::Error;

fn main() -> Result<(), Box<Error>> {

    let mut runtime = tokio::runtime::Runtime::new()?;

    let camera = ks2_timelapse::Camera::new();
    let timer_interval = Duration::from_secs(15);

    let photo_stream = Interval::new_interval(timer_interval).from_err().and_then(move |_interval| {
        camera.take_photo()
    });

    let photo_stream = photo_stream.for_each(|r| {
        println!("took photo: {:?}", r);
        Ok(())
    })
    .map_err(|e| {
        println!("{:?}", e);
    });

    runtime.spawn(photo_stream);

    runtime.shutdown_on_idle().wait().unwrap();
    Ok(())
}


