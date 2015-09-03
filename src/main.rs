extern crate iron;
//extern crate hyper;

use iron::prelude::*;
use iron::status;
//use hyper::header::ContentType;
use iron::mime::Mime;

fn main() {
    fn hello_world(_: &mut Request) -> IronResult<Response> {
        let html = "text/html".parse::<Mime>().unwrap();
        Ok(Response::with((status::Ok, html, "<!doctype html>\nHello World!")))
    }

    Iron::new(hello_world).http("localhost:3000").unwrap();
    println!("On 3000");
}
