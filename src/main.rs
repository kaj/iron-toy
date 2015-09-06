extern crate iron;
#[macro_use]
extern crate router;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use router::Router;

macro_rules! render {
    ($template:expr, $($name:ident=$arg:tt),*) => {{
        let html = "text/html".parse::<Mime>().unwrap();
        Ok(Response::with((status::Ok, html,
                           format!($template, $($name=$arg),*))))
    }};
}

fn main() {
    fn hello_world(req: &mut Request) -> IronResult<Response> {
        let (who,) = {
            let args = req.extensions.get::<Router>().unwrap();
            (args.find("who").unwrap_or("World").to_string(),)
        };
        render!("<!doctype html>\nHello {who}!", who=who)
    }
    fn other(req: &mut Request) -> IronResult<Response> {
        let (x, y) = {
            let args = req.extensions.get::<Router>().unwrap();
            ( args.find("x").unwrap_or("World").to_string(),
              args.find("y").unwrap_or("walk").to_string(),
              )
        };
        render!("<!doctype html>\nHello {who}!<p>Fine day for a {what}.",
                who=x, what=y)
    }
    Iron::new(router!(
        get "/" => hello_world,
        get "/foo/:who" => hello_world,
        get "/bar/" => other,
        get "/bar/:x" => other,
        get "/bar/:x/:y" => other
        )).http("localhost:3000").unwrap();
    println!("On 3000");
}
