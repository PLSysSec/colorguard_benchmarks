// use fastly::{Error, Request, Response};
use hashring::HashRing;

fn main() {
  //Initialize hashring
  let mut ring: HashRing <&str> = HashRing::new();
  ring.add("origin_0");
  ring.add("origin_1");
  ring.add("origin_2");
  //use query to get hash
  let query = "/page/1337";
//   let query = req.get_query_str().unwrap_or("string");
  let backend = ring.get(&query).unwrap();

  //debugging...
  println!("query: {}", query);
  println!("Sending to backend {} ...", backend);  
}