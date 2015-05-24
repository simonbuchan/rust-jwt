use rustc_serialize::json;
use rustc_serialize::base64::FromBase64;
use error::Error;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Header {
    pub typ: String,
    pub alg: Option<String>,
}

impl Header {
    pub fn parse(raw: &str) -> Result<Header, Error> {
        let data = try!(raw.from_base64());
        let s = try!(String::from_utf8(data));
        let header = try!(json::decode(&*s));

        Ok(header)
    }
}
