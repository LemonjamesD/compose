use std::{collections::HashMap, sync::Mutex};

use lazy_static::lazy_static;

lazy_static! {
    pub static ref HASHMAP_SYSTEMS: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}
