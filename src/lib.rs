use regex::Regex;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::StatusCode;
use serde::Serialize;
use std::error::Error;
use std::time::SystemTime;

mod constants;

pub struct Listen<'a> {
    pub artist: &'a str,
    pub track: &'a str,
    pub token: &'a str,
    pub malojaserver: &'a str,
}

impl Listen<'_> {
    pub fn single(&self, uuid_user: &str) -> Result<StatusCode, Box<dyn Error>> {
        let time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let song = Submission {
            artist: self.artist,
            title: self.track,
            key: self.token,
        };
        match submit_single(uuid_user, self.malojaserver, song) {
            Ok(status_code) => Ok(status_code),
            Err(e) => Err(e),
        }
    }
}
#[derive(Serialize, Default)]
pub struct Submission<'a> {
    pub artist: &'a str,
    pub title: &'a str,
    pub key: &'a str,
}

pub fn submit_single(user_uuid: &str, malojaserver: &str, sub: Submission) -> Result<StatusCode, Box<dyn Error>> {
        let mut headers = HeaderMap::new();
        let mut auth = String::from("Token ");
        auth.push_str(user_uuid);
        headers.insert(AUTHORIZATION, HeaderValue::from_str(auth.as_str()).unwrap());
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        let j = serde_json::to_string(&sub).unwrap();
        let res = reqwest::Client::new()
            .post(malojaserver)
            .headers(headers)
            .body(j)
            .send()?;
        return Ok(res.status());
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}