use anyhow::{Context, Result, anyhow};
use tokio::sync::{broadcast::self, mpsc};
use tokio_tungstenite::{
    connect_async, 
    tungstenite::protocol::Message,
};

use futures_util::StreamExt;
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::str::FromStr;
use actix::Addr;
use crate::CONFIG;
use crate::actix_actors::FeedActor;
use super::settings::*;
use super::web_socket::*;
use super::types::*;

pub async fn stream_init_task(feed_actor_addr: Addr<FeedActor>) -> Result<()>
{
    let task_name = "--Stream Init Task--";



    let (ws_stream, _) = connect_async(CONFIG.web_socket_url.clone()).await
        .context(format!("Error in {:?}:\ninput_rx_ch:\n", task_name))?;

    let (writer, reader) = ws_stream.split();

    let (writer_tx_ch, writer_rx_ch): (mpsc::Sender<Message>, mpsc::Receiver<Message>) = mpsc::channel(20);
    
    let writer_settings = WriterSettings::new(writer, writer_rx_ch);
    tokio::spawn(writer_task(writer_settings));

    let (reader_tx_ch, reader_rx_ch) = broadcast::channel(10);
    let reader_settings = ReaderSettings::new(reader, reader_tx_ch);
    tokio::spawn(reader_task(reader_settings));

    writer_tx_ch.send(CONFIG.web_socket_feed.clone()).await
    .context(format!("Error in {:?}:\ninput_rx_ch:\n", task_name))?;

    let (output_tx_ch, mut output_rx_ch) =  broadcast::channel(10);    
    let deserialize_settings = DeserializeSettings::new(reader_rx_ch, output_tx_ch, writer_tx_ch);
    tokio::spawn(stream_management_task(deserialize_settings));
    
    while let Ok(message) = output_rx_ch.recv().await{

        if message.luna_exch_rates.len() > 0 {
            feed_actor_addr.do_send(message);
        }
        
    }
    Ok(())
}

async fn stream_management_task(mut deserialize_settings: DeserializeSettings){
    let task_name = "--Stream Management Task--";
    log::info!("{:?} Init", task_name);
    loop{
        match websocket_msg_process(&mut deserialize_settings).await {
            Ok(_)=> continue,
            Err(err) => {
                log::error!("{:?}", err);

                match &err.downcast_ref::<broadcast::error::RecvError>() {
                    Some(err) => {
                        match err {
                            broadcast::error::RecvError::Lagged(x) => {
                                log::trace!("Trace in {:?}:\ninput_rx_ch lagged:\n{:?}\n", task_name, x); 
                                continue;
                            },
                            broadcast::error::RecvError::Closed => {
                                log::warn!("Warning in {:?}:\ninput_rx_ch closed:\n", task_name); 
                                break;
                            }
                
                        }
                    },
                    None =>  log::warn!("Warning in {:?}:\ninput_rx_ch closed:\n", task_name)
                };          
            }
        };
    } 
    log::info!("{:?} End", task_name);
}

async fn websocket_msg_process(deserialize_settings: &mut DeserializeSettings) -> Result<()> {
    let task_name = "--websocket msg process Task--";

    let input_msg = deserialize_settings.input_rx_ch.recv().await
        .context(format!("Error in {:?}:\ninput_rx_ch:\n", task_name))?;

    log::trace!("{:?}:\nReceived message from reader\n{:?}", task_name, input_msg);

    match input_msg {
            
        Message::Close(close_data) => log::warn!("Warning in {:?}:\nClose message received:\n {:?}", task_name, close_data),

        Message::Ping(ping_data) => {
            let pong_msg = Message::Pong(ping_data);

            deserialize_settings.writer_tx_ch.send(pong_msg).await
                .context(format!("Error in {:?}:\nSending pong:\n", task_name))?;
            log::trace!("Trace in {:?}:\nSent pong", task_name)
        },

        Message::Pong(pong_data) => log::warn!("Warning in {:?}:\nPong message received:\n {:?}", task_name, pong_data),

        Message::Text(text_data) => {
            let data = deserialize_stream(text_data)
                .context(format!("Error in {:?}:\nDesirializing:\n", task_name))?;

            deserialize_settings.output_tx_ch.send(data)
                .context(format!("Error in {:?}:\nSending data:\n", task_name))?;

        },

        Message::Binary(_) => log::warn!("Warning in {:?}: binary data sent:\n", task_name)
    }

    Ok(())

}

pub fn deserialize_stream(json_str: String) -> Result<OutputData>{
    log::info!("Deserialize stream Init");

    let feed: Feed = serde_json::from_str(&json_str)
        .context("JSON was not well-formatted deserialize_stream")?;

    //Aggregate all events from all logs from all tx
    let events_iter = feed.data.txs.into_iter()
        .flat_map(|x| x.logs)
        .flat_map(|x| x.events);

    // let borrow_stable_iter = events_iter.clone().filter(|x| x.event_type == "borrow_stable");
    //Aggregate all events from all logs
    //Could use filter_map() but using first filter then map is more readable, avoids an ugly 'if' clause  
    let rates_iter = events_iter
        .filter(|x| x.event_type == "aggregate_vote")
        .map(|x| x.attributes.into_iter()
                    .filter(|y| y.key == "exchange_rates")
                    .map(|x| x.value))
        .flatten();

    let mut hash_map: HashMap<Asset, ExchangeRate> = HashMap::new();
    //Only need one voter exchange rate.
    for i in rates_iter.take(1){
        let split = i.split(",");
        for attribute in split.into_iter() {

            let reg_symbol = regex::Regex::new(r"([a-z]+)")?;
    
            let reg_value = regex::Regex::new(r"([+-]?\d+\.?\d*|\.\d+)")?;

            let symbol: &str = reg_symbol.find(&attribute)
                                .ok_or(anyhow!("Error regex symbol"))?.as_str();
            let rate: &str = reg_value.find(&attribute)
                                .ok_or(anyhow!("Error regex rate"))?.as_str();

            hash_map.insert(symbol.to_owned(), Decimal::from_str(rate)?);
        }
    }
  
    let output_data = OutputData{
        luna_exch_rates: hash_map,
        // events: borrow_stable_iter.collect::<Vec<_>>()
    };

    Ok(output_data)
}