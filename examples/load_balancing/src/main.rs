// use fastly::{Error, Request, Response};
use hashring::HashRing;
use std::time;
use std::thread::sleep;
use rand::thread_rng;
use rand::Rng;
use rand_distr::{Poisson, Distribution};
// fn delay_time() -> u64 {
//   1000
// }

fn delay() {
  // let t = delay_time();
  // let ten_millis = time::Duration::from_millis(10);
  // sleep(ten_millis);
  let lambda = 10.0;
  let poi = Poisson::new(lambda).unwrap();
  let v = poi.sample(&mut rand::thread_rng());
  //println!("v = {:?} {:?}", v, v as u64);
  let millis = time::Duration::from_millis(v as u64);
  sleep(millis);
}

fn main() {

  //delay();

  let mut rng = thread_rng();
  let n: u16 = rng.gen();
  //let num_iterations = 10;
  // let chosen = rng.gen::<usize>() % num_iterations;
  //for idx in 0..num_iterations {
  //if idx == chosen {
    delay();
  //}
      // println!("{:x}", n);
  //Initialize hashring
  let mut ring: HashRing <&str> = HashRing::new();
  ring.add("origin_0");
  ring.add("origin_1");
  ring.add("origin_2");
  //use query to get hash
  let query = format!("/page/{n}");
  //println!("query: {}", query);

//   let query = req.get_query_str().unwrap_or("string");
  let backend = ring.get(&query).unwrap();


  delay();
  //debugging...
  //println!("Sending {n} to backend {backend} ...");  
  // println!("{:x}", n);
  //}
}
