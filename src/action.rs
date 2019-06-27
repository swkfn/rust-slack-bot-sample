extern crate slack;

use self::slack::RtmClient;

pub fn respond(bot_id: &str, text: &str, channel: &str, cli: &RtmClient) {
    let pat_hi = format!("<@{}> {}", bot_id, "hi");
    let pat_1 = format!("<@{}> {}", bot_id, "good");

    if text.contains(&pat_hi) {
        let _ = cli.sender().send_message(channel, "Hi!");
    }
    if text.contains(&pat_1){
        let _ = cli.sender().send_message(channel, ":+1:");
    }
}
