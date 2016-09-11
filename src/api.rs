extern crate hyper;
extern crate crypto;
extern crate random;
extern crate url;

use self::hyper::client::response::Response;
use self::random::Source;
use self::crypto::md5::Md5;
use self::crypto::digest::Digest;
use std::ascii::AsciiExt;
use std::io::Read;

fn get(url: &str) -> Response {
    hyper::Client::new()
        .get(url)
        .send()
        .unwrap()
}

fn detect_lang(content: &str) -> &'static str {
    if content.is_ascii() {
        "zh"
    } else {
        "en"
    }
}

fn random_salt() -> u64 {
    random::default().read_u64()
}

fn sign(appid: &str, query: &str, salt: u64, secret: &str) -> String {
    let sign_string = format!("{}{}{}{}", appid, query, salt, secret);
    let mut sh = Md5::new();
    sh.input_str(&sign_string);
    sh.result_str().to_owned()
}

pub fn translate(content: &str, app_id: &str, secret: &str) -> String {
    let query = content;
    let from = "auto";
    let to = detect_lang(content);
    let salt = random_salt();
    let sign = sign(app_id, query, salt, secret);
    let url = format!("http://api.fanyi.baidu.com/api/trans/vip/translate?appid={}&q={}&from={}&to={}&salt={}&sign={}", app_id, query, from, to, salt, sign);
    let mut ripen = String::new();
    get(&url).read_to_string(&mut ripen).unwrap();
    ripen
}