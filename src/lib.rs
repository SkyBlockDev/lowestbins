pub mod bazaar;
pub mod fetch;
pub mod nbt_utils;
pub mod server;

use std::collections::HashMap;
use std::env;
use std::{fs, sync::Mutex, time::Duration};

use surf::{Client, Config};

lazy_static::lazy_static! {
   pub static ref AUCTIONS: Mutex<HashMap<String, u64>> ={
      let defaults = include_bytes!(concat!(env!("OUT_DIR"), "/sellprices.json"));
      let mut res: HashMap<String, u64> = fs::read("auctions.json")
            .map(|x| serde_json::from_slice(&x).unwrap())
            .unwrap_or_default();
      let map = serde_json::from_slice::<HashMap<String, f64>>(defaults).unwrap();
      res.extend(map.into_iter().map(|(k, v)| (k, v.round() as u64)));
      Mutex::new(res)
   };
   pub static ref HTTP_CLIENT: Client = Config::new()
        .set_timeout(Some(Duration::from_secs(50)))
        .try_into()
        .unwrap();
   pub static ref OVERWRITES: HashMap<String,u64> = {
      let overwrites = env::var("OVERWRITES").unwrap_or("".to_string());
      let mut map = HashMap::new();
      for overwrite in overwrites.split(",") {
         let mut split = overwrite.split(":");
         let key = split.next().unwrap();
         if let Some(value) = split.next() {
            map.insert(key.to_string(), value.parse().unwrap());
         }
      }
      map
   };
}
