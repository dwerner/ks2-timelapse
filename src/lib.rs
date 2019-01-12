extern crate futures;
extern crate hyper;
extern crate tokio;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

pub mod ks2;

use futures::{
    Future,
    Stream,
};

use std::path::PathBuf;

use ks2::ShootResponse;

#[derive(Debug)]
pub struct Camera {
    ip: String,
    out_dir: PathBuf,
}

#[derive(Debug)]
pub enum FetchError {
    Http(hyper::Error),
    Json(serde_json::Error),
    Timer(tokio::timer::Error),
}

impl From<hyper::Error> for FetchError {
    fn from(err: hyper::Error) -> FetchError {
        FetchError::Http(err)
    }
}

impl From<serde_json::Error> for FetchError {
    fn from(err: serde_json::Error) -> FetchError {
        FetchError::Json(err)
    }
}

impl From<tokio::timer::Error> for FetchError {
    fn from(err: tokio::timer::Error) -> FetchError {
        FetchError::Timer(err)
    }
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            ip: "192.168.0.1".to_string(),
            out_dir: "output".into()
        }
    }

    // Ideally, take a photo, download it to the fs, delete it, and return path to it.
    pub fn take_photo(&self) -> impl Future<Item=ShootResponse, Error=FetchError> {
        let client = hyper::Client::new();
        let url = format!("http://{}/camera/shoot", self.ip);
        let mut req = hyper::Request::builder();
        req.uri(url);

        let req = Box::new(
            client.request(req.body(hyper::Body::empty()).unwrap())
            .and_then(|res| {
                res.into_body().concat2()
            })
            .from_err::<FetchError>()
            .and_then(|body| {
                let shoot_response = serde_json::from_slice(&body)?;
                Ok(shoot_response)
            }).from_err()
        );

        req
    }
}
