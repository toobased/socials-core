use std::{net::{TcpListener, TcpStream, SocketAddr}, time::{Duration, SystemTime}};

use log::debug;
use fantoccini::ClientBuilder;
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
        // run webdriver instance on auto-port
        let res = process::Command::new("geckodriver")
            .arg("--log")
            .arg("error")
            .arg("-p")
            .arg(socket.port().to_string())
            .spawn().expect("Failed to spawn geckodriver");
        res
        /*
        let addr = SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(127,0,0,1)),
            4444
        );
        */
    }

    async fn init_client (socket: &SocketAddr) -> fantoccini::Client {
        let mut client = ClientBuilder::native();
        // "args": ["-headless"],
        let capabilities = r#"{
                "moz:firefoxOptions": {
                "args": ["-headless"],
                "prefs": {
                    "media.volume_scale": "0.0"
                },
                "log": {"level": "fatal"}
            }
        }"#;
        let cap = serde_json::from_str(capabilities).unwrap();
        client.capabilities(cap);

        // let mut url: String = "http://127.0.0.1:".to_string();
        // url.push_str(port);
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
