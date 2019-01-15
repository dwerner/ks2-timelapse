extern crate ks2_timelapse;
extern crate tokio;
extern crate futures;

#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

use std::error::Error;
use std::io::BufReader;
use std::fs::File;
use std::time::{ Duration, Instant };
use futures::{ Future, Stream, };
use tokio::timer::Interval;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {

    #[serde(default)]
    pub delay: u64,

    pub output: String,
}

impl Default for Config {
    fn default() -> Config {
        Config{ delay: 60, output: "out".to_string() }
    }
}

impl Config {
    pub fn load_file(path: &str) -> Result<Self, Box<Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Ok(serde_yaml::from_reader(reader)?)
    }
}

fn main() -> Result<(), Box<Error>> {

    let mut runtime = tokio::runtime::Runtime::new()?;

    let path = "config.yaml";
    let config = Config::load_file(path)?;
    println!("Loaded config {} : {:?}", path, config);
    let camera = ks2_timelapse::Camera::new();

    let timer_interval = Duration::from_secs(config.delay);

    let photo_stream = Interval::new_interval(timer_interval).from_err().and_then(move |_interval| {
        camera.take_photo()
    });

    let photo_stream = photo_stream.for_each(|r| {
        let now = Instant::now();
        println!("took photo at {:?}", now);
        println!("response {:#?}", r);
        Ok(())
    })
    .map_err(|e| {
        println!("{:?}", e);
    });

    runtime.spawn(photo_stream);
    runtime.shutdown_on_idle().wait().unwrap();
    Ok(())
}


