use eventsource::reqwest::Client;
use reqwest::Url;
use serde_json::Value;

fn main() {
    const STREAM_URL: &str = "http://status.maxiv.lu.se/stream";
    let client = Client::new(Url::parse(STREAM_URL).unwrap());
    let mut r3_current = 0f64;
    let mut r3_lifetime = 0f64;
    let mut r1_current = 0f64;
    let mut r1_lifetime = 0f64;
    let mut spf_charge = 0f64;

    let mut got_r3_lifetime = false;
    let mut got_r1_lifetime = false;
    let mut got_r3_current = false;
    let mut got_r1_current = false;
    let mut got_spf_charge = false;

    for event in client {
        let data = &event.as_ref().unwrap().data;
        let v: Value = serde_json::from_str(data).unwrap();
        if let Some(val) = v["R3-319S2/DIA/DCCT-01/CURRENT"]["value"].as_f64() {
            r3_current = val * 1e3;
            got_r3_current = true;
        }
        if let Some(val) = v["R3-319S2/DIA/DCCT-01/LIFETIME"]["value"].as_f64() {
            r3_lifetime = val / (60f64 * 60f64);
            got_r3_lifetime = true;
        }
        if let Some(val) = v["R1-101S/DIA/DCCT-01/CURRENT"]["value"].as_f64() {
            r1_current = val * 1e3;
            got_r1_current = true;
        }
        if let Some(val) = v["R1-101S/DIA/DCCT-01/LIFETIME"]["value"].as_f64() {
            r1_lifetime = val / (60f64 * 60f64);
            got_r1_lifetime = true;
        }
        if let Some(val) = v["I-SP02/DIA/CT-02/AVERAGECHARGE"]["value"].as_f64() {
            spf_charge = val;
            got_spf_charge = true;
        }
        if got_r3_current && got_r1_current && got_r3_lifetime && got_r1_lifetime && got_spf_charge
        {
            break;
        }
    }
    println!(
        "R3: {:.1} mA ({:.1} h) :: R1: {:.1} mA ({:.1} h) :: SPF {} C",
        r3_current, r3_lifetime, r1_current, r1_lifetime, spf_charge
    );
}
