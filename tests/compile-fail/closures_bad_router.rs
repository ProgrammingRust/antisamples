// If the scope of a type parameter is too broad, too many things end up being
// forced to be the same type.

use std::collections::HashMap;

struct Request {
    method: String,
    url: String,
    headers: HashMap<String, String>,
    body: Vec<u8>
}

struct Response {
    code: u32,
    headers: HashMap<String, String>,
    body: Vec<u8>
}

struct BasicRouter<C> where C: Fn(&Request) -> Response {
    routes: HashMap<String, C>
}

impl<C> BasicRouter<C> where C: Fn(&Request) -> Response {
    /// Create an empty router.
    fn new() -> BasicRouter<C> {
        BasicRouter { routes: HashMap::new() }
    }

    /// Add a route to the router.
    fn add_route(&mut self, url: &str, callback: C) {
        self.routes.insert(url.to_string(), callback);
    }
}

fn get_form_response() -> Response { unimplemented!(); }
fn get_gcd_response(_req: &Request) -> Response { unimplemented!(); }

fn main() {
    let mut router = BasicRouter::new();
    router.add_route("/", |_| get_form_response());
    router.add_route("/gcd", |req| get_gcd_response(req));
    //~^ERROR: mismatched types
    //~|NOTE: expected closure, found a different closure
    //~|NOTE: expected type
    //~|NOTE: no two closures, even if identical, have the same type
    //~|HELP: consider boxing your closure and/or using it as a trait object
}
