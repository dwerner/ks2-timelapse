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

use ks2::{
    ShootResponse,
    PhotosResponse,
};

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
    Timeout,
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

impl <T> From<tokio::timer::timeout::Error<T>> for FetchError {
    // We're choosing to ignore any details of this error for now - it shouldn't be practically
    // relevant
    fn from(_err: tokio::timer::timeout::Error<T>) -> FetchError {
        FetchError::Timeout
    }
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            ip: "192.168.0.1".to_string(),
            out_dir: "output".into()
        }
    }

    pub fn get_files(&self) -> impl Future<Item=PhotosResponse, Error=FetchError> {
        println!("getting photo list from {}", self.ip);
        let client = hyper::Client::new();
        let url = format!("http://{}/v1/photos", self.ip);
        let mut req = hyper::Request::get(url);
        let req = Box::new(
            client.request(req.body(hyper::Body::empty()).unwrap())
            .and_then(|res| {
                res.into_body().concat2()
            })
            .from_err::<FetchError>()
            .and_then(|body| {
                Ok(serde_json::from_slice(&body)?)
            }).from_err()
        );
        req
    }

    // Ideally, take a photo, download it to the fs, delete it, and return path to it.
    pub fn take_photo(&self) -> impl Future<Item=ShootResponse, Error=FetchError> {
        println!("taking photo at {}", self.ip);
        let client = hyper::Client::new();
        let url = format!("http://{}/v1/camera/shoot", self.ip);
        let mut req = hyper::Request::post(url);
        let req = Box::new(
            client.request( req.body(hyper::Body::empty()).unwrap() )
            .and_then(|res| {
                res.into_body().concat2()
            })
            .from_err::<FetchError>()
            .and_then(|body| {
                //TODO: actually write the file to disk, return json and a filename?
                //let content = body.to_vec();
                //let resp = String::from_utf8(content.clone()).unwrap();
                //println!("response: {}", resp);
                Ok(serde_json::from_slice(&body)?)
            }).from_err()
        );
        req
    }
}
