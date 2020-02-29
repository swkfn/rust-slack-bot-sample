extern crate slack_api;

//use rust_slack_bot_sample::handle;
use slack_api as slack;

use std::default::Default;
use std::{fs, process};

fn main() {
    let token: String = api_key();
    let client =
        slack::default_client().unwrap();
    let request = slack::rtm::StartRequest::default();
    let response = slack::rtm::start(&client, &token, &request);
    //let mut handler = handle::MyHandle;
    println! {"{:?}", response};
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
