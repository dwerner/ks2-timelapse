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
};

use std::path::PathBuf;

use tokio::timer::Interval;
use tokio::runtime::Runtime;

pub struct Camera {
    ip: String,
    out_dir: PathBuf,
}

enum FetchError {
    Http(hyper::Error),
    Json(serde_json::Error)
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

impl Camera {
    pub fn new() -> Self {
        PhotoStream {
            ip: "192.168.0.1".to_string(),
            out_dir: "output".into()
        }
    }

    pub fn take_photo(&self) -> impl Future<Item=ShootResponse, Error=FetchError> {
        let client = hyper::Client::new();
        let req = hyper::client::Request::new();
        client.request((format!("http://{}/camera/shoot", self.ip))
            .and_then(|res| res.into_body().concat2() )
            .from_err::<FetchError>()
            .and_then(|body| {
                let shoot_response = serde_json::from_slice(&body)?;
                Ok(shoot_response)
            }).from_err()
    }
}
