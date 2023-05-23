use eventsource::reqwest::Client;
use reqwest::Url;
use serde_json::Value;

fn main() {
    const STREAM_URL: &str = "http://status.maxiv.lu.se/stream";
    let client = Client::new(Url::parse(STREAM_URL).unwrap());
    let mut r3_info = 0f64;
    let mut r1_info = 0f64;

    for event in client {
        let data = &event.as_ref().unwrap().data;
        let v: Value = serde_json::from_str(data).unwrap();
        if let Some(new_r3_info) = v["R3-319S2/DIA/DCCT-01/CURRENT"]["value"].as_f64() {
            r3_info = new_r3_info * 1e3;
        }
        if let Some(new_r1_info) = v["R1-101S/DIA/DCCT-01/CURRENT"]["value"].as_f64() {
            r1_info = new_r1_info * 1e3;
        }
        println!("R3: {:.1} mA :: R1: {:.1} mA", r3_info, r1_info);
    }
}
