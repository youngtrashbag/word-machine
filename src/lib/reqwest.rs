use reqwest::blocking::{get, Client};
use reqwest::{redirect::Policy, Proxy};
use std::time::Duration;

pub fn build_client() -> Client {
    Client::builder()
        .timeout(Duration::from_secs(10))
        .redirect(Policy::none())
        //.redirect(Policy::limited(1))
        // default tor proxy
        //.proxy(Proxy::all("socks5h://127.0.0.1:9050").expect("Tor Proxy not running"))
        .build().unwrap()
}
