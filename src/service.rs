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

#[cfg(test)]
extern crate mockito;

#[cfg(not(test))]
const URL: &'static str = "https://circleci.com";

#[cfg(test)]
const URL: &'static str = mockito::SERVER_URL;


pub fn fetch_projects() -> Result<Vec<Project>, Error> {
    let tls = NativeTlsClient::new()?;
    let connector = HttpsConnector::new(tls);
    let client = Client::with_connector(connector);
    let url = format!("{}/api/v1/projects?circle-token={}", URL, dotenv!("CIRCLECI_TOKEN"));
    let mut res = client.get(&url)
    .header(Accept(vec![
        qitem(Mime(TopLevel::Application, SubLevel::Json,vec![])),
    ]))
    .send()?;

    let mut body = String::new();
    res.read_to_string(&mut body)?;

    json::decode(&body).map_err(|e| From::from(e))
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use super::mockito::mock;
    use super::*;

    #[test]
    fn valid_json_response() {
        let json = r#"
        [
            {
                "irc_server": null,
                "ssh_keys": [],
                "branches": {
                    "master": {
                        "recent_builds": [
                            {
                                "outcome": "success",
                                "build_num": 72235,
                                "vcs_revision": "fadfadfafe93939fsafsd",
                                "pushed_at": "2016-11-11T07:50:41.418Z"
                            }
                        ]
                    }
                },
                "reponame": "mzumi.github.io",
                "username": "mzumi",
                "vcs_url": "https://github.com/mzumi/mzumi.github.io"
            }
        ]
        "#;
        let url = format!("/api/v1/projects?circle-token={}", dotenv!("CIRCLECI_TOKEN"));
        mock("GET", url.as_str()).with_body(json).create_for(|| {
            let projects = fetch_projects().unwrap();
            let ref project = projects[0];
            assert_eq!("mzumi", project.username);
            assert_eq!("mzumi.github.io", project.reponame);
            assert_eq!("https://github.com/mzumi/mzumi.github.io", project.vcs_url);

            let branch = project.branches.get("master").unwrap();
            if let Some(ref recent_builds) = branch.recent_builds {
                let ref recent_build = recent_builds[0];
                assert_eq!("success", recent_build.outcome);
                assert_eq!("2016-11-11T07:50:41.418Z", recent_build.pushed_at);
                assert_eq!(72235, recent_build.build_num);
                assert_eq!("fadfadfafe93939fsafsd", recent_build.vcs_revision);
            }
        });
    }

    #[test]
    fn invalid_json_response() {
        let json = r#"
        
            {
                "irc_server": null,
                "ssh_keys": [],
                "branches": {
                    "master": 
                        "recent_builds": [
                            {
                                "outcome": "success",
                                "build_num": 72235,
                                "vcs_revision": "fadfadfafe93939fsafsd",
                                "pushed_at": "2016-11-11T07:50:41.418Z"
                            }
                        ]
                    }
                },
                "reponame": "mzumi.github.io",
                "username": "mzumi",
                "vcs_url": "https://github.com/mzumi/mzumi.github.io"
            }
        ]
        "#;
        let url = format!("/api/v1/projects?circle-token={}", dotenv!("CIRCLECI_TOKEN"));
        mock("GET", url.as_str()).with_body(json).create_for(|| {
            println!("start invalid");
            let projects = fetch_projects();
            assert!(projects.is_err());
            assert_eq!(projects.err().unwrap().description(), "decoder error");
        });
    }

    #[test]
    fn connection_error() {
        let projects = fetch_projects();
        assert!(projects.is_err());
        assert_eq!(projects.err().unwrap().description(), "connection refused");
    }
}