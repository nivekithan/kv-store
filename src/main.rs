use rocket::{futures::lock::Mutex, Config, State};
use std::collections::HashMap;

#[macro_use]
extern crate rocket;

#[get("/<key>")]
async fn get_key(key: &str, kv_store: &State<Mutex<HashMap<String, String>>>) -> String {
    let store_lock = kv_store.lock().await;
    let value = store_lock.get(key);

    match value {
        Some(v) => v.to_string(),
        None => format!("Not avaliable 2"),
    }
}

#[post("/<key>", data = "<value>")]
async fn set_key(
    key: &str,
    value: &str,
    kv_store: &State<Mutex<HashMap<String, String>>>,
) -> String {
    let mut store_lock = kv_store.lock().await;

    store_lock.insert(key.to_string(), value.to_string());

    return format!("Insert {key} with value {value}");
}

#[launch]
fn rocket() -> _ {
    let args: Vec<String> = std::env::args().collect();

    println!("{args:?}");

    let port = args.get(1).unwrap().parse::<u16>().unwrap();

    let mut rocket_config = Config::debug_default();
    rocket_config.port = port;

    rocket::custom(rocket_config)
        .mount("/", routes![get_key, set_key])
        .manage(Mutex::new(HashMap::<String, String>::new()))
}
