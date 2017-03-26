extern crate dotenv;

use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use hyper::header::{Accept, qitem};
use hyper::mime::{Mime, TopLevel, SubLevel};
use rustc_serialize::json;

use std::io::Read;

use response::*;
use error::*;

pub fn fetch_projects() -> Result<Vec<Project>, Error> {
    let tls = NativeTlsClient::new()?;
    let connector = HttpsConnector::new(tls);
    let client = Client::with_connector(connector);
    let url = format!("https://circleci.com/api/v1/projects?circle-token={}", dotenv!("CIRCLECI_TOKEN"));
    let mut res = client.get(&url)
    .header(Accept(vec![
        qitem(Mime(TopLevel::Application, SubLevel::Json,vec![])),
    ]))
    .send()?;

    let mut body = String::new();
    res.read_to_string(&mut body)?;

    json::decode(&body).map_err(|e| From::from(e))
}
