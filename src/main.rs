extern crate rust_slack_bot_sample;
extern crate slack;

use rust_slack_bot_sample::handle;
use self::slack::RtmClient;

use std::{fs, process};

fn main() {
    let api_key: String = api_key();
    let mut handler = handle::MyHandle;
    let r = RtmClient::login_and_run(&api_key, &mut handler);
    match r {
        Ok(_) => {},
        Err(err) => panic!("Error: {}", err)
    }
}

fn api_key() -> String {
    let content = fs::read_to_string("slack_api");
    match content {
        Ok(content) => content,
        Err(_) => {
            println!("Required the SLACK_API_TOKEN file");
            process::exit(1);
        }
    }
}