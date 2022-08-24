use std::{net::{SocketAddr, IpAddr, Ipv4Addr, TcpListener}, process};

use async_trait::async_trait;
use log::info;

use crate::{social::SocialCore, tasks::{BotTask, watch::WatchAction}};
use fantoccini::{ClientBuilder, Locator, wd::Capabilities};

pub struct BrowserCore {
    webdriver_process: process::Child,
    // listener: TcpListener,
    pub client: fantoccini::Client
}

impl BrowserCore {
    pub async fn new () -> Self { Self::init().await }

    /*
    fn get_listener () -> Option<TcpListener> {
        for port in 4446..5000 {
            match TcpListener::bind(("127.0.0.1", port)) {
                Ok(l) => return Some(l),
                _ => {}
            }
        }
        None
    }
    */

    fn get_free_port () -> Option<String> {
        for port in 4446..5000 {
            match TcpListener::bind(("127.0.0.1", port)) {
                Ok(_) => return Some(port.to_string()),
                _ => {}
            }
        }
        None
    }

    async fn init_driver (port: &str) -> process::Child {
        // run webdriver instance on auto-port
        let res = process::Command::new("geckodriver")
            .arg("--log")
            .arg("error")
            .arg("-p")
            .arg(port)
            .spawn().expect("Failed to spawn geckodriver");
        res
        /*
        let addr = SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(127,0,0,1)),
            4444
        );
        */
    }

    async fn init_client (port: &str) -> fantoccini::Client {
        let mut client = ClientBuilder::native();
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

        let mut url: String = "http://localhost:".to_string();
        url.push_str(port);
        info!("connection url is {}", url);
        let client = client
            .connect(&url)
            .await
            .expect("Cant connect to browser client");
        info!("Browser client initalized on port 'port_here'");
        client
    }

    // TODO handle error
    pub async fn init () -> Self {
        // let listener = Self::get_listener().expect("Cant raise listener.");
        // listener.set_nonblocking(true).unwrap();
        // let port = listener.local_addr().unwrap().port().to_string();
        let port = BrowserCore::get_free_port().expect("Cant get free port");
        // let port = "4445";
        let webdriver = Self::init_driver(&port).await;
        Self {
            webdriver_process: webdriver,
            // listener,
            client: Self::init_client(&port).await
        }
    }

    pub async fn close_client(client: fantoccini::Client) {
        client.close().await.expect("cant close client");
    }

    pub async fn close_webdriver(process: &mut process::Child) {
        process.kill().expect("Failed to kill webdriver process.");
    }

    pub async fn close (self) {
        // client.close().await.expect("Failed to close client.");
        let mut process = self.webdriver_process;
        BrowserCore::close_client(self.client).await;
        BrowserCore::close_webdriver(&mut process).await;
    }
}

#[async_trait]
impl SocialCore for BrowserCore {
    fn info (&self) -> String {
        "BrowserCore".to_string()
    }
}
