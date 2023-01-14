use std::collections::HashMap;

use bdk::bitcoin::{Address, Amount};

// A rust function that maps over a hashmap and turns them in the url search params
fn hashmap_to_searchparams(map: HashMap<&str, &str>) -> String {
    map.iter()
        .map(|(key, value)| format!("{key}={value}"))
        .collect::<Vec<String>>()
        .join("&")
}

pub fn create_bip_21(address: Address, bolt11: String, amount: Amount, label: String) -> String {
    let mut map: HashMap<&str, &str> = HashMap::new();
    let amount = amount.to_btc().to_string();
    map.insert("amount", &amount);
    map.insert("label", &label);
    map.insert("lightning", &bolt11);
    let params = hashmap_to_searchparams(map);
    format!("bitcoin:{address}?{params}")
}
