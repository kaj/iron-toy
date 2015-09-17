extern crate iron;
#[macro_use]
extern crate router;
extern crate handlebars_iron as hbs;
extern crate rustc_serialize;
extern crate logger;

use std::collections::BTreeMap;
use std::thread::sleep_ms;
use iron::prelude::*;
use iron::status;
use router::Router;
use hbs::{Template, HandlebarsEngine};
use rustc_serialize::json::ToJson;
use logger::Logger;

macro_rules! render {
    ($template:expr, $($name:ident=$arg:expr),*) => {{
        let mut data = BTreeMap::new();
        $( data.insert(stringify!($name).to_string(), $arg.to_json()); )*
        Ok(Response::with((status::Ok, Template::new($template, data))))
    }};
}

fn main() {
    fn hello_world(req: &mut Request) -> IronResult<Response> {
        let (who,) = {
            let args = req.extensions.get::<Router>().unwrap();
            (args.find("who").unwrap_or("World").to_string(),)
        };
        render!("hello", who=who)
    }

    fn other(req: &mut Request) -> IronResult<Response> {
        let (x, y) = {
            let args = req.extensions.get::<Router>().unwrap();
            ( args.find("x").unwrap_or("World").to_string(),
              args.find("y").unwrap_or("walk").to_string(),
              )
        };
        sleep_ms(2000);
        render!("hello", who=x, what=y)
    }

    let mut chain = Chain::new(router!(
        get "/" => hello_world,
        get "/foo/:who" => hello_world,
        get "/bar/" => other,
        get "/bar/:x" => other,
        get "/bar/:x/:y" => other
            ));
    chain.link_after(HandlebarsEngine::new("./templates/", ".hbs"));
    chain.link(Logger::new(None)); // last => includes template rendering time
    let app = Iron::new(chain);
    let addr = "localhost:3000";
    println!("Running on http://{}", addr);
    app.http("localhost:3000").unwrap();
}
