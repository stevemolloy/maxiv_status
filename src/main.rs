use eventsource::reqwest::Client;
use reqwest::Url;

fn main() {
    const URL: &str = "http://status.maxiv.lu.se/stream";
    let client = Client::new(Url::parse(URL).unwrap());

    for event in client {
        println!("{}", event.unwrap());
    }
}
