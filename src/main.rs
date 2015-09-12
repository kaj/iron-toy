extern crate iron;
#[macro_use]
extern crate router;
extern crate liquid;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use router::Router;
use std::fs::File;
use std::io::Read;
use liquid::LiquidOptions;
use liquid::Renderable;
use liquid::Context;
use liquid::Value;
use liquid::Template;
use liquid::parse;

fn load_template(name: &str) -> Result<Template, String> {
    let filename = format!("./templates/{}.html", name);
    let mut text = String::new();
    let _x = File::open(filename).unwrap().read_to_string(&mut text);
    let mut options : LiquidOptions = Default::default();
    parse(&text, &mut options)
}

macro_rules! render {
    ($template:expr, $($name:ident=$arg:expr),*) => {{
        //println!("Should render {:?} {:?}", $template, vec!($(stringify!($name)),*));
        let html = "text/html".parse::<Mime>().unwrap();
        let template = match load_template($template) {
            Ok(result) => result,
            Err(e) => panic!("Failed to parse template {:?}: {}", $template, e)
        };

        let mut data = Context::new();
        $(
            data.set_val(stringify!($name), $arg);
           )*
        match template.render(&mut data) {
            Some(text) => Ok(Response::with((status::Ok, html, text.to_string()))),
            None => panic!("Failed to render template {}", $template)
        }
    }};
}

fn main() {
    fn hello_world(req: &mut Request) -> IronResult<Response> {
        let (who,) = {
            let args = req.extensions.get::<Router>().unwrap();
            (args.find("who").unwrap_or("World").to_string(),)
        };
        render!("hello", who=Value::Str(who))
    }
    fn other(req: &mut Request) -> IronResult<Response> {
        let (x, y) = {
            let args = req.extensions.get::<Router>().unwrap();
            ( args.find("x").unwrap_or("World").to_string(),
              args.find("y").unwrap_or("walk").to_string(),
              )
        };
        render!("hello", who=Value::Str(x), what=Value::Str(y))
    }
    let app = Iron::new(router!(
        get "/" => hello_world,
        get "/foo/:who" => hello_world,
        get "/bar/" => other,
        get "/bar/:x" => other,
        get "/bar/:x/:y" => other
    ));
    let addr = "localhost:3000";
    println!("Running on http://{}", addr);
    app.http("localhost:3000").unwrap();
}
