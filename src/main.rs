extern crate ks2_timelapse;
extern crate tokio;
extern crate futures;

use futures::{
    Future,
    Stream,
};

use std::error::Error;

fn main() -> Result<(), Box<Error>> {

    let mut runtime = tokio::runtime::Runtime::new()?;

    let camera = ks2_timelapse::Camera::new();

    let photo_future = camera.take_photo().map(|r| {
        println!("took photo: {:?}", r);
    }).map_err(|e| println!("{:?}", e) );

    runtime.spawn(photo_future);

    runtime.shutdown_on_idle().wait().unwrap();
    Ok(())
}
