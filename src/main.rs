use reqwest::Url;
use std::{env, thread::sleep, time::Duration};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let delta: u64 = args[1].parse().unwrap();
    let delta = Duration::from_millis(delta);

    let url = if let Ok(url) = Url::parse(&args[2]) {
        url
    } else {
        println!("Bad url");
        return;
    };

    loop {
        let response = reqwest::get(url.clone()).await.unwrap();
        println!("Checking: '{}' Result({})", url, response.status());

        sleep(delta);
    }
}
