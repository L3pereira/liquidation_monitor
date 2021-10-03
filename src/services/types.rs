use serde::{Serialize, Deserialize, de::Error, Deserializer};
use anyhow::{Result, Context};
use rust_decimal::Decimal;
use std::collections::HashMap;
use url::Url;
use tokio_tungstenite::tungstenite::protocol::Message;
use actix::Message as ActixMessage;


#[derive(Clone, Debug, Deserialize)]
pub struct Feed{
    pub chain_id: String,

    #[serde(alias = "type")]
    pub block_type: String,

    pub data: FeedData
}

#[derive(Clone, Debug, Deserialize)]
pub struct FeedData{
    pub txs: Vec<FeedDataTx>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct FeedDataTx{
    pub logs: Vec<FeedDataTxLog>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct FeedDataTxLog{
    pub events: Vec<FeedDataTxLogEvent>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FeedDataTxLogEvent{

    #[serde(alias = "type")]
    pub event_type: String,

    pub attributes: Vec<FeedDataTxLogEventAttribute>,  

}

#[derive(Clone, Debug, Serialize,  Deserialize)]
pub struct FeedDataTxLogEventAttribute{

    pub key: String,  

    pub value: String
}

/////////////////////////////////////////////////////////Output Data Feed//////////////////////////////////////////////////////
pub type Asset = String;
pub type ExchangeRate = Decimal;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ActixMessage)]
#[rtype(result = "()")]
pub struct OutputData{

    pub luna_exch_rates: HashMap<Asset, ExchangeRate>
    
}

/////////////////////////////////////////////////////////Server Config//////////////////////////////////////////////////////

#[derive(Clone, Debug, Deserialize)]
pub struct RawConfig{
    pub web_socket_url: String,
    
    pub web_socket_feed: String,

    pub server_address: String,

    pub server_port: u16
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WebAppConfig{

    pub web_socket_url: Url,

    pub web_socket_feed: Message,

    pub server_address: String,

    pub server_port: u16
}
impl WebAppConfig {
    pub fn new(json_str: String) -> Result<Self>{

        let config: WebAppConfig = serde_json::from_str(&json_str)
            .context("JSON was not well-formatted config")?;

        Ok(config)

    } 
}
impl<'de> Deserialize<'de> for WebAppConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw_config: RawConfig = Deserialize::deserialize(deserializer)?;
        Ok(WebAppConfig{
            web_socket_url:  Url::parse(&raw_config.web_socket_url).map_err(D::Error::custom)?,
            web_socket_feed: Message::text(raw_config.web_socket_feed),
            server_address: raw_config.server_address,
            server_port: raw_config.server_port
        })

    }
}