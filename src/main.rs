extern crate iron;
#[macro_use]
extern crate router;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use router::Router;

fn main() {
    fn hello_world(req: &mut Request) -> IronResult<Response> {
        let html = "text/html".parse::<Mime>().unwrap();
        let ref query = req.extensions.get::<Router>()
            .unwrap().find("query").unwrap_or("World");
        Ok(Response::with((status::Ok, html,
                           format!("<!doctype html>\nHello {}!", query))))
    }
    fn other(req: &mut Request) -> IronResult<Response> {
        let html = "text/html".parse::<Mime>().unwrap();
        let ref x = req.extensions.get::<Router>()
            .unwrap().find("x").unwrap_or("Client");
        Ok(Response::with((status::Ok, html,
                           format!("<!doctype html>\nHello {}!", x))))
    }
    Iron::new(router!(
        get "/" => hello_world,
        get "/foo/:query" => hello_world,
        get "/bar/:x" => other
        )).http("localhost:3000").unwrap();
    println!("On 3000");
}
