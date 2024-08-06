use eventsource::reqwest::Client;
use reqwest::Url;
use serde_json::Value;

const STREAM_URL: &str = "http://status.maxiv.lu.se/stream";
const R3_CURR_ID: &str = "R3-319S2/DIA/DCCT-01/CURRENT";
const R1_CURR_ID: &str = "R1-101S/DIA/DCCT-01/CURRENT";
const SPF_CHARGE_ID: &str = "I-SP02/DIA/CT-02/AVERAGECHARGE";

fn main() {
    let client = Client::new(Url::parse(STREAM_URL).unwrap());

    let mut r3_current = "---".to_string();
    let mut r1_current = "---".to_string();
    let mut spf_charge = "---".to_string();

    for event in client {
        let data = &event.as_ref().unwrap().data;
        let v: Value = serde_json::from_str(data).unwrap();
        if let Some(val) = v[R3_CURR_ID]["value"].as_f64() {
            r3_current = format!("{:.1}", val * 1e3);
        }
        if let Some(val) = v[R1_CURR_ID]["value"].as_f64() {
            r1_current = format!("{:.1}", val * 1e3);
        }
        if let Some(val) = v[SPF_CHARGE_ID]["value"].as_f64() {
            spf_charge = format!("{:.1}", val * 1e12);
        }
        println!(
            "| R3 {} mA | R1 {} mA | SPF {} pC",
            r3_current, r1_current, spf_charge
        );
        break;
    }
}
