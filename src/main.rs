use eventsource::reqwest::Client;
use reqwest::Url;
use serde_json::Value;

fn main() {
    const STREAM_URL: &str = "http://status.maxiv.lu.se/stream";
    let client = Client::new(Url::parse(STREAM_URL).unwrap());

    for event in client {
        let data = &event.as_ref().unwrap().data;
        let relevant = data.contains("R3-319S2/DIA/DCCT-01/CURRENT");
        if relevant {
            let v: Value = serde_json::from_str(data).unwrap();
            let r3_info = v["R3-319S2/DIA/DCCT-01/CURRENT"]["value"].as_f64().unwrap();
            println!("R3: {:.0} mA", r3_info * 1e3);
        }
    }
}
