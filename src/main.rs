#[path = "twitch/twitch_reader.rs"]
mod twitch_reader;
#[path = "twitch/twitch_client.rs"]
mod twitch_client;
#[path = "twitch/twitch_vod.rs"]
mod twitch_vod;
#[path = "twitch/twitch_channel.rs"]
mod twitch_channel;
#[path = "twitch/twitch_clip.rs"]
mod twitch_clip;
mod tools;


use std::io::{stdin, stdout, Write};
use crate::twitch_clip::print_clips_from;

fn main() {
    let mut platform_name = String::new();
    print!("What platform would you link to pull from (Twitch)? >>> ");
    stdout()
        .flush()
        .expect("Could not flush line when preparing for <vod_link>");
    stdin()
        .read_line(&mut platform_name)
        .expect("Could not read response for <vod_link>");
    platform_name = String::from(platform_name.trim_end_matches(&['\r', '\n'][..]));

    if platform_name.eq_ignore_ascii_case("Twitch") {
        twitch_reader::main()
    } else {
        eprintln!("\n'{}' was an unexpected response\nPlease choose between [Twitch]\n", platform_name);
        main()
    }
}
