extern crate slack_api;

/*
use self::slack::RtmClient;

pub fn respond(text: &str, channel: &str, cli: &RtmClient, user: &str) {
    let pat_hi = format!("{}", "bot hi");
    let pat_1 = format!("{}", "bot good");

    if text.contains(&pat_hi) {
        let reply = format!("<@{}> {}", user, "hi");
        let _ = cli.sender().send_message(channel, &reply);
    }
    if text.contains(&pat_1){
        let _ = cli.sender().send_message(channel, ":+1:");
    }
}
*/