use std::io::Read;

use hyper::{Client, Url};
use hyper::header;
use hyper::error::Error;


pub struct JenkinsTask {
    pub badge_url: Url,
}


impl JenkinsTask {

    pub fn new(badge_url: &str) -> Result<JenkinsTask, String> {
        match Url::parse(badge_url) {
            Ok(val) => Ok(JenkinsTask { badge_url: val }),
            Err(e) => Err(format!("Could not parse URL: {} ({})", badge_url, e)),
        }
    }

    pub fn is_failing(&self) -> Result<bool, String> {
        match self.get_body() {
            Err(e) => Err(format!("Could not download the status badge: {}", e)),
            Ok(body) => {
                if body.contains("failing") {
                    Ok(true)
                } else if body.contains("passing") {
                    Ok(false)
                } else {
                    Err("Could not determine build status from badge".to_string())
                }
            }
        }
    }

    // Get the HTTP body of the build badge
    fn get_body(&self) -> Result<String, Error> {
        let client = Client::new();
        let mut res = try!(client.get(self.badge_url.clone()).send());
        let mut body = String::new();
        try!(res.read_to_string(&mut body));
        Ok(body)
    }

}
