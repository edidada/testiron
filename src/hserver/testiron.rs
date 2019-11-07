extern crate iron;

use iron::prelude::*;
use iron::status;


pub fn testiron(){
    fn mian_page(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "This Is Rust Http Server")))
    }

    let _server = Iron::new(mian_page).http("127.0.0.1:3000").unwrap();
}