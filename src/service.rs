extern crate rustc_serialize;
extern crate dotenv;

use self::rustc_serialize::json::{self};

use hyper::Client;
use hyper::header::{Accept, qitem};
use hyper::mime::{Mime, TopLevel, SubLevel};

use std::io::Read;

pub fn fetch_projects() -> json::Json {
    let client = Client::new();
    let url = format!("https://circleci.com/api/v1/projects?circle-token={}", dotenv!("CIRCLECI_TOKEN"));
    let mut res = client.get(&url)
    .header(Accept(vec![
        qitem(Mime(TopLevel::Application, SubLevel::Json,vec![])),
    ]))
    .send().unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    json::Json::from_str(&body).unwrap()
}
