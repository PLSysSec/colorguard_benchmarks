// use fastly::{Error, Request, Response};
use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::time;
use std::thread::sleep;
use rand_distr::{Poisson, Distribution};


fn delay() {
    // let t = delay_time();
    // let ten_millis = time::Duration::from_millis(10);
    // sleep(ten_millis);
    let lambda = 5.0;
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
        // Capture a group with brackets and regex::captures[n]
        lazy_static! {
            static ref RE: Regex = Regex::new(r"/status/(\d+)").unwrap();
        }
        if let Some(caps) = RE.captures(url) {
            assert_eq!(&caps[1], "200");
            log::info!("Captured status code using positional groups: {}", &caps[1]);
        }
    }

    {
        // Named captures with (?P<name>exp)
        lazy_static! {
            static ref RE: Regex = Regex::new(r"/status/(?P<code>\d+)").unwrap();
        }
        if let Some(caps) = RE.captures(url) {
            assert_eq!(&caps["code"], "200");
            log::info!("Captured status code using named groups: {}", &caps["code"]);
        }
    }

    {
        // Non-capturing groups can be created by prefixing the
        // bracketed expression with ?:
        lazy_static! {
            static ref RE: Regex = Regex::new(r"/(?:status|code)/(\d+)\?").unwrap();
        }
        if let Some(caps) = RE.captures(url) {
            assert_eq!(&caps[1], "200");
            log::info!("Captured status code using non-capturing groups: {}", &caps[1]);
        }
    }

    {
        // Bracketed character class with [...]
        lazy_static! {
            static ref RE: Regex = Regex::new(r"[?&]aaa=([^&]*)").unwrap();
        }
        if let Some(caps) = RE.captures(url) {
            assert_eq!(&caps[1], "foo");
            log::info!("Captured aaa parameter using bracketed character class: {}", &caps[1]);
        }
    }

    {
        // Specific repeat count with {min,max}
        // Alternate options using (a|b)
        lazy_static! {
            static ref RE: Regex = Regex::new(r"[\?&]a{2,6}=(foo|bar)(?:$|&)").unwrap();
        }
        if let Some(caps) = RE.captures(url) {
            assert_eq!(&caps[1], "foo");
            log::info!("Captured aaa parameter using specific repeat count and alternate options: {}", &caps[1]);
        }
    }

    {
        // Make the match case insensitive using an (?i) prefix
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?i)AaA=Foo").unwrap();
        }
        if let Some(caps) = RE.captures(url) {
            assert_eq!(&caps[0], "aaa=foo");
            log::info!("Case insensitive match using (?i)AaA=Foo: {}", &caps[0]);
        }
    }
    println!("Done!");
    Ok(())
}
