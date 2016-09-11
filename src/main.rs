extern crate toml;
extern crate rustc_serialize;

use std::fs::File;
use std::io::Read;
use toml::Value;
use rustc_serialize::json;

mod api;

#[derive(Debug)]
struct Config {
    app_id: String,
    secret: String,
}

impl Config {
    fn new(config_path: &str) -> Self {
        let mut file_content = String::new();
        File::open(config_path)
            .and_then(|mut f| f.read_to_string(&mut file_content))
            .unwrap();

        let mut parser = toml::Parser::new(&file_content);
        let toml = parser.parse().unwrap();
        let traitor_section = toml.get("traitor").unwrap_or_else(|| {
            panic!("config error: no `traitor` section in {}", config_path)
        });
        let traitor_section = if let &Value::Table(ref traitor_section) = traitor_section {
            traitor_section
        } else {
            panic!("section `traitor` is NOT a table in {}", file_content)
        };
        let app_id = traitor_section.get("APP_ID").unwrap_or_else(|| {
            panic!("config error: no `APP_ID` in `traitor` section")
        });
        let app_id = if let &Value::String(ref app_id) = app_id {
            app_id
        } else {
            panic!("`APP_ID` is NOT a String in `traitor` section")
        };
        let secret = traitor_section.get("SECRET").unwrap_or_else(|| {
            panic!("config error: no `SECRET` in `traitor` section")
        });
        let secret = if let &Value::String(ref secret) = secret {
            secret
        } else {
            panic!("`SECRET` is NOT a String in `traitor` section")
        };
        Config {
            app_id: app_id.clone(),
            secret: secret.clone(),
        }
    }
}

#[derive(RustcDecodable)]
struct TransResult {
    trans_result: Vec<ResultItem>,
}

#[derive(RustcDecodable)]
struct ResultItem {
    dst: String,
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    assert!(args.len() == 2);
    let config_path = format!("{}/.traitor", std::env::home_dir().unwrap().to_str().unwrap());
    let conf = Config::new(&config_path);
    let json_content = api::translate(&args[1], &conf.app_id, &conf.secret);
    let decoded: TransResult = json::decode(&json_content).unwrap();
    for item in decoded.trans_result {
        println!("{}", item.dst);
    }
}
