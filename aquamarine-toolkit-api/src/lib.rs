use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};
use serde::{Serialize, Deserialize};
use log::{debug, error, trace};

const CLIENT_CONFIG_FILE: &'static str = "aquamarine-toolkit-client.json";

pub struct PluginDefinition {
    pub name: String,
    pub initialize: Box<dyn Fn()>,
    pub handler_function: Box<dyn Fn(Vec<u8>)>
}

#[derive(Serialize, Deserialize)]
pub enum AquamarineMessage {
    Signal {
        target: String,
        data: Vec<u8>
    }
}

pub fn send_message_to_server(from: String, data: Vec<u8>) -> Result<(), AquamarineError> {
    let config = load_client_config()?;
    let socket = UdpSocket::bind(["0.0.0.0:0"].into_iter().map(|a| a.to_socket_addrs().unwrap().next().unwrap()).collect::<Vec<_>>().as_slice());
    if socket.is_err() {
        error!("Unable to bind socket");
    }
    let socket = socket?;
    debug!("Connecting to \"{}\"", &config.host);
    socket.connect(config.host).map_err(|a| {error!("Failed to connect: {}", &a); a})?;
    let data = bincode::serialize(&AquamarineMessage::Signal {target: from, data}).unwrap();
    socket.send(&data)?;
    Ok(())
}

fn load_client_config() -> Result<ClientConfig, AquamarineError> {
    let path = dirs::config_dir().map(|a| a.join(CLIENT_CONFIG_FILE));
    trace!("Loading config file: {:?}", path);
    if let None = path {
        error!("Failed to determine config file path.");
        return Err(AquamarineError::NoConfigFile);
    }
    let path = path.unwrap();
    if !path.exists() {
        debug!("Creating default config...");
        // not existing
        let file = File::create(&path)?;
        let res = serde_json::to_writer_pretty(file, &ClientConfig::default());
        if res.is_err() {
            error!("Failed to write default config.");
            res?;
        }
    }
    trace!("Reading config file...");
    // not exists or is dir
    if path.is_file() {
        let file = File::open(&path)?;
        let config = serde_json::from_reader::<_, ClientConfig>(file)?;
        Ok(config)
    } else {
        // is directory
        error!("Config file is a directory: {:?}", path);
        Err(AquamarineError::NoConfigFile)
    }

}


#[derive(Deserialize, Serialize)]
struct ClientConfig {
    host: SocketAddr
}

impl Default for ClientConfig {
    fn default() -> Self {
        ClientConfig {
            host: "127.0.0.1:5555".to_socket_addrs().unwrap().next().unwrap()
        }
    }
}

#[derive(Debug)]
pub enum AquamarineError {
    NoConfigFile,
    IOError(std::io::Error),
    JSONError(serde_json::Error)
}

impl Display for AquamarineError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoConfigFile => write!(f, "No config file!"),
            Self::IOError(err) => write!(f, "IO Error: {}", err),
            Self::JSONError(err) => write!(f, "JSON Error: {}", err),
        }
    }
}

impl Error for AquamarineError {}

impl From<std::io::Error> for AquamarineError {
    fn from(err: std::io::Error) -> Self {
        AquamarineError::IOError(err)
    }
}

impl From<serde_json::Error> for AquamarineError {
    fn from(err: serde_json::Error) -> Self {
        AquamarineError::JSONError(err)
    }
}
