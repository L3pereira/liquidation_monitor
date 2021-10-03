// use anyhow::{Context, Result};
use anyhow:: Result;
use futures_util::{
    SinkExt, 
    StreamExt,
    stream::Stream,
    sink::Sink
};

use tokio_tungstenite::{
    tungstenite::protocol::Message,
    tungstenite::error::Error as WsError
};

use super::settings::{ReaderSettings, WriterSettings};

pub async fn reader_task<S>(mut settings: ReaderSettings<S>) 
where S: Stream<Item=Result<Message, WsError>> + Unpin {
    let task_name = "--Reader Task--";
    log::info!("{:?} Init", task_name);   
    while let Some(message) = settings.websocket_reader.next().await {      
        match message {
            Ok(message) => {
                log::trace!("{:?}:\n{:?}", task_name, message);
                if let Err(err) = settings.output_tx_ch.send(message) {
                    log::error!("Error in {:?}\noutput_tx_ch:\n{:?}", task_name, err);
                    break;
                }
            },
            Err(err) => log::error!("Error in {:?}:\nReading message from stream:\n{:?}", task_name, err)
        }       
    }
    log::info!("{:?} End", task_name);
}

pub async fn writer_task<S>(mut settings: WriterSettings<S>) 
    where S: Sink<Message, Error= WsError>  + Unpin
{
    let task_name = "--Writer Task--";
    log::info!("{:?} Init", task_name);
    while let Some(message) = settings.input_rx_ch.recv().await { 
        if let  Err(err) = settings.websocket_writer.send(message.clone()).await {
            log::error!("Error in {:?}:\nSending message to stream:\n{:?}", task_name, err)
        }
        else{
            log::trace!("{:?}:\n{:?}", task_name, message);
        }
    }
    log::info!("{:?} End", task_name);

}


