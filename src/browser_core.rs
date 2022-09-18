use std::{net::{TcpListener, TcpStream, SocketAddr}, time::{Duration, SystemTime}, env };

use log::debug;
use fantoccini::ClientBuilder;
use serde_json::json;
use tokio::process;

#[cfg(test)]
pub mod tests;

pub struct BrowserCore {
    webdriver_process: process::Child,
    pub webdriver_socket: SocketAddr,
    pub client: fantoccini::Client
}

impl BrowserCore {
    pub async fn new () -> Self { Self::init().await }

    fn default_max_watch_spawn () -> u64 { 4 }

    pub fn get_max_watch_spawn () -> u64 {
        let d = BrowserCore::default_max_watch_spawn();
        match env::var("webdriver_max_spawn") {
            Ok(v) => v.parse::<u64>().unwrap_or(d),
            _ => d
        }
    }

    pub fn is_headless () -> bool {
        let d = true;
        match env::var("socials_headless") {
            Ok(v) => match v.parse::<u64>().unwrap_or(1) {
                0 => false,
                _ => d
            },
            _ => d
        }
    }

    fn get_free_socket () -> Option<SocketAddr> {
        let listener = TcpListener::bind(("127.0.0.1", 0));
        match listener {
            Ok(l) => Some(l.local_addr().unwrap()),
            _ => None
        }
    }

    async fn wait_driver_initialized (addr: &SocketAddr) {
        debug!("Waiting for driver server on {}...", addr);
        let timeout = Duration::from_secs(15);
        let fail_time = SystemTime::now().checked_add(timeout).unwrap();
        let interval_sleep = Duration::from_secs(1);
        loop {
            if SystemTime::now().ge(&fail_time) {
                panic!("Cant connect to geckodriver for {:#?}s", interval_sleep)
            }
            match TcpStream::connect(addr) {
                Ok(_s) => break,
                Err(_) => {}
            }
            tokio::time::sleep(interval_sleep).await
        }
    }

    fn init_driver (socket: &SocketAddr) -> process::Child {
        let res = process::Command::new("geckodriver")
            .arg("--log")
            .arg("fatal")
            .arg("-p")
            .arg(socket.port().to_string())
            .spawn().expect("Failed to spawn geckodriver");
        res
    }

    async fn init_client (socket: &SocketAddr) -> fantoccini::Client {
        let mut client = ClientBuilder::native();
        let mut args: Vec<&str> = Vec::new();
        args.push("-private");
        if BrowserCore::is_headless() { args.push("-headless") }

        let capabilities = json!({
            "moz:firefoxOptions": {
                "args": args,
                "prefs": {
                    "media.volume_scale": "0.0"
                },
                "log": {"level": "error"},
                "env": {
                    "RUST_LOG": "error"
                }
            }
        });
        let cap = serde_json::from_value(capabilities).unwrap();
        debug!("Capabilities are {:#?}", cap);
        client.capabilities(cap);
        debug!("Client try to connect with socket {}", socket.to_string());
        let mut url = String::from("http://");
        url.push_str(&socket.to_string());
        let client = client
            .connect(&url)
            .await
            .expect("Cant connect to browser client");
        debug!("Browser client initalized on socket {}", socket.to_string());
        client
    }

    // TODO handle error
    pub async fn init () -> Self {
        let addr = BrowserCore::get_free_socket().expect("Cant get free port");
        debug!("Socket will be used: {}", addr.to_string());
        let webdriver = Self::init_driver(&addr);
        BrowserCore::wait_driver_initialized(&addr).await;
        Self {
            webdriver_process: webdriver,
            webdriver_socket: addr,
            // listener,
            client: Self::init_client(&addr).await
        }
    }

    pub async fn close_client(client: fantoccini::Client) {
        client.close().await.expect("cant close client");
    }

    pub async fn close_webdriver(process: &mut process::Child) {
        process.kill().await.expect("Failed to kill webdriver process.");
    }

    pub async fn close (self) {
        // client.close().await.expect("Failed to close client.");
        let mut process = self.webdriver_process;
        BrowserCore::close_client(self.client).await;
        BrowserCore::close_webdriver(&mut process).await;
    }
}
