use std::{
    str::FromStr,
    collections::HashMap
};
use url::Url;
use tokio_tungstenite::tungstenite::protocol::Message;
use rust_decimal::Decimal;
use pretty_assertions::assert_eq;
use crate::services::types::{WebAppConfig, OutputData, Asset, ExchangeRate};
use crate::services::data_processor::deserialize_stream;

#[test]
fn test_deserialize_config(){
    let data = r#"{
        "web_socket_url": "wss://observer.terra.dev",
        "web_socket_feed": "{\"subscribe\": \"new_block\", \"chain_id\": \"tequila-0004\"}",
        "server_address": "127.0.0.1",
        "server_port": 8080
    }"#;

    
    let expected =  WebAppConfig{
        web_socket_url:  Url::parse("wss://observer.terra.dev").unwrap(),
        web_socket_feed: Message::text("{\"subscribe\": \"new_block\", \"chain_id\": \"tequila-0004\"}"),
        server_address: "127.0.0.1".to_string(),
        server_port: 8080_u16
    };
    
    let result= WebAppConfig::new(data.to_string()).unwrap();
    assert_eq!(expected, result);
}

#[test]
fn test_deserialize_stream(){
    // super::setup();
    let data =  r#"
    {
        "chain_id": "columbus-4",
        "type": "new_block",
        "data": {
            "txs": [ 
                {
                    "logs": [
                        {
                          "msg_index": 0,
                          "log": "",
                          "events": [
                                {
                                    "type": "aggregate_vote",
                                    "attributes": [
                                        {
                                            "key": "voter",
                                            "value": "terravaloper10lk5rrz3srq28ap6x68c9hs86zvvtpm0jkn5qh"
                                        },
                                        {
                                            "key": "exchange_rates",
                                            "value": "52.634119837760564508uaud,48.352968234552155517ucad,35.59545083416239871uchf,246.466768644419850917ucny,245.257640387642196568udkk,33.005740201513804404ueur,28.243996927431822597ugbp,297.644465482137718753uhkd,2834.493685892844001877uinr,4246.205482740957679647ujpy,45105.738551278812757673ukrw,108846.146106784313515381umnt,330.039235451102953546unok,27.096290779791758406usdr,334.494135897282764838usek,51.898507853232289836usgd,1284.094820624236417267uthb,38.245397968611556217uusd"
                                        },
                                        {
                                            "key": "feeder",
                                            "value": "terra1t6elycv27d4qd3n6fecv6fxwd23sacq8dg4rn5"
                                        }
                                    ]
                                },
                                {
                                    "type": "message",
                                    "attributes": [
                                        {
                                            "key": "action",
                                            "value": "aggregateexchangeratevote"
                                        },
                                        {
                                            "key": "module",
                                            "value": "oracle"
                                        }
                                    ]
                                }
                            ]
                        },
                        {
                            "msg_index": 1,
                            "log": "",
                            "events": [
                                    {
                                       "type": "aggregate_prevote",
                                        "attributes": [
                                            {
                                                "key": "voter",
                                                "value": "terravaloper10lk5rrz3srq28ap6x68c9hs86zvvtpm0jkn5qh"
                                            },
                                            {
                                                "key": "feeder",
                                                "value": "terra1t6elycv27d4qd3n6fecv6fxwd23sacq8dg4rn5"
                                            }
                                        ]
                                    },
                                    {
                                        "type": "message",
                                        "attributes": [
                                            {
                                                "key": "action",
                                                "value": "aggregateexchangerateprevote"
                                            },
                                            {
                                                "key": "module",
                                                "value": "oracle"
                                            }
                                        ]
                                    }
                                ]
                        }
                    ]
                }
            ]
        }          
    }"#; 

    let mut hash_map: HashMap<Asset, ExchangeRate> = HashMap::new();
    
    hash_map.insert("udkk".to_owned(), Decimal::from_str("245.257640387642196568").unwrap());
    hash_map.insert("ugbp".to_owned(), Decimal::from_str("28.243996927431822597").unwrap());
    hash_map.insert("uhkd".to_owned(), Decimal::from_str("297.644465482137718753").unwrap());
    hash_map.insert("ujpy".to_owned(), Decimal::from_str("4246.205482740957679647").unwrap());
    hash_map.insert("uaud".to_owned(), Decimal::from_str("52.634119837760564508").unwrap());
    hash_map.insert("umnt".to_owned(), Decimal::from_str("108846.146106784313515381").unwrap());
    hash_map.insert("unok".to_owned(), Decimal::from_str("330.039235451102953546").unwrap());
    hash_map.insert("usdr".to_owned(), Decimal::from_str("27.096290779791758406").unwrap());
    hash_map.insert("ucad".to_owned(), Decimal::from_str("48.352968234552155517").unwrap());
    hash_map.insert("ucny".to_owned(), Decimal::from_str("246.466768644419850917").unwrap());
    hash_map.insert("usgd".to_owned(), Decimal::from_str("51.898507853232289836").unwrap());
    hash_map.insert("uchf".to_owned(), Decimal::from_str("35.59545083416239871").unwrap());
    hash_map.insert("uthb".to_owned(), Decimal::from_str("1284.094820624236417267").unwrap());
    hash_map.insert("uusd".to_owned(), Decimal::from_str("38.245397968611556217").unwrap());
    hash_map.insert("uinr".to_owned(), Decimal::from_str("2834.493685892844001877").unwrap());
    hash_map.insert("ukrw".to_owned(), Decimal::from_str("45105.738551278812757673").unwrap());
    hash_map.insert("usek".to_owned(), Decimal::from_str("334.494135897282764838").unwrap());
    hash_map.insert("ueur".to_owned(), Decimal::from_str("33.005740201513804404").unwrap());
    
    let expected = OutputData{
        luna_exch_rates: hash_map
    };
    let result =  deserialize_stream(data.to_string()).unwrap();
    println!("{:?}", result);
    assert_eq!(expected, result);

}