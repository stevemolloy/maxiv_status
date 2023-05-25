use eventsource::reqwest::Client;
use reqwest::Url;
use serde_json::Value;

fn main() {
    const STREAM_URL: &str = "http://status.maxiv.lu.se/stream";
    let client = Client::new(Url::parse(STREAM_URL).unwrap());

    const R3_CURR_ID: &str = "R3-319S2/DIA/DCCT-01/CURRENT";
    const R1_CURR_ID: &str = "R1-101S/DIA/DCCT-01/CURRENT";
    const SPF_CHARGE_ID: &str = "I-SP02/DIA/CT-02/AVERAGECHARGE";

    let mut r3_current = 0f64;
    let mut r1_current = 0f64;
    let mut spf_charge = 0f64;

    let mut got_r3_current = false;
    let mut got_r1_current = false;
    let mut got_spf_charge = false;

    for event in client {
        let data = &event.as_ref().unwrap().data;
        let v: Value = serde_json::from_str(data).unwrap();
        if let Some(val) = v[R3_CURR_ID]["value"].as_f64() {
            r3_current = val * 1e3;
            got_r3_current = true;
        }
        if let Some(val) = v[R1_CURR_ID]["value"].as_f64() {
            r1_current = val * 1e3;
            got_r1_current = true;
        }
        if let Some(val) = v[SPF_CHARGE_ID]["value"].as_f64() {
            spf_charge = val * 1e12;
            got_spf_charge = true;
        }
        if got_r3_current && got_r1_current && got_spf_charge {
            break;
        }
    }
    println!(
        "R3 {:.1} mA / R1 {:.1} mA / SPF {:.1} pC",
        r3_current, r1_current, spf_charge
    );
}
