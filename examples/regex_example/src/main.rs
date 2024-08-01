// use fastly::{Error, Request, Response};
use anyhow::Result;
//use lazy_static::lazy_static;
//use regex::Regex;
use std::time;
use std::thread::sleep;
use rand_distr::{Poisson, Distribution};
use mr_regex::regex_match;

fn delay() {
    // let t = delay_time();
    // let ten_millis = time::Duration::from_millis(10);
    // sleep(ten_millis);
    let lambda = 10.0;
    let poi = Poisson::new(lambda).unwrap();
    let v = poi.sample(&mut rand::thread_rng());
    let millis = time::Duration::from_millis(v as u64);
    sleep(millis);
  }
 


// #[fastly::main]
fn main() -> Result<()> {
    delay();

    // log_fastly::init_simple("mylogs", log::LevelFilter::Info);
    // let url = req.get_url_str();
    let url = "www.example.com/status/200";
    {
        { 
        //let re = Regex::new(r"/status/(\d+)").unwrap();
        
        if regex_match(r"/status/(\d+)", url)  { //re.captures(url) {
            log::info!("Url has status code");
        }
    }
    }
    delay();
    //println!("Done!");
    Ok(())
}
