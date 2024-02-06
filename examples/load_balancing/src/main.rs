// use fastly::{Error, Request, Response};
use hashring::HashRing;
use std::time;
use std::thread::sleep;
use rand::thread_rng;
use rand::Rng;
// fn delay_time() -> u64 {
//   1000
// }

fn delay() {
  // let t = delay_time();
  let ten_millis = time::Duration::from_millis(10);
  sleep(ten_millis);
}

fn main() {
  let mut rng = thread_rng();
  let n: u16 = rng.gen();

  // println!("{:x}", n);
  //Initialize hashring
  let mut ring: HashRing <&str> = HashRing::new();
  ring.add("origin_0");
  ring.add("origin_1");
  ring.add("origin_2");
  //use query to get hash
  let query = format!("/page/{n}");
  println!("query: {}", query);
  delay();

//   let query = req.get_query_str().unwrap_or("string");
  let backend = ring.get(&query).unwrap();

  //debugging...
  println!("Sending {n} to backend {backend} ...");  
  // println!("{:x}", n);
}