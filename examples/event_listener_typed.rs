use bytes::{BufMut, BytesMut};
use compact_str::CompactString;
use futures::stream::StreamExt;
use std::collections::HashMap;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio_util::codec::{Decoder, Encoder};
use tokio_util::codec::{FramedRead, FramedWrite};
use thiserror::Error;

#[allow(unused)]

pub mod types;
pub mod codec;






#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listen = std::env::args().nth(1).expect("Missing LISTEN");
    let mut stream = TcpStream::connect(&listen).await.unwrap();

    let message = b"auth ClueCon\n\n";
    stream.write_all(message).await.unwrap();

    let message = b"event json ALL\n\n";
    stream.write_all(message).await.unwrap();

    let in_codec = EslCodec::default();
    let mut framed_read = FramedRead::new(stream, in_codec);

    while let Some(Ok(data)) = framed_read.next().await {

        use crate::types::EslEvent;
        use crate::types::EslCallDirection;

        if let Ok(jdata) = serde_json::from_str::<EslEvent>(&data.payload.unwrap_or_default()) {
            match jdata {

                EslEvent::ChannelCreate { channel, caller, event, other, .. } => {
                    println!("[{}] new      [{}] {} -> {}", event.date_gmt, if channel.call_direction == EslCallDirection::Inbound { "inbound " } else { "utbound" } , caller.channel_name, channel.name );
                },

                EslEvent::ChannelOutgoing { channel, caller, event, other, .. } => {
                    println!("[{}] trying   [{}] {} -> {}", event.date_gmt, if channel.call_direction == EslCallDirection::Inbound { "inbound " } else { "outbound" } , caller.channel_name, channel.name );
                },

                EslEvent::ChannelAnswer { channel, caller, event, other, .. } => {
                    println!("[{}] answer   [{}] {} -> {}", event.date_gmt, if channel.call_direction == EslCallDirection::Inbound { "inbound " } else { "outbound" } , caller.channel_name, channel.name );
                },

                EslEvent::ChannelHangup { channel, caller, event, other, .. } => {
                    println!("[{}] hangup   [{}] {} -> {}", event.date_gmt, if channel.call_direction == EslCallDirection::Inbound { "inbound " } else { "outbound" } , caller.channel_name, channel.name );
                },

                EslEvent::Heartbeat { freeswitch, event, up_time, .. } => {
                    println!("[{}][<3] {}", event.date_gmt, up_time );
                },

                any => { dbg!(any); },
            }
        }

/*
        if let Ok(event_data) = serde_json::from_str::<HashMap<CompactString,CompactString>>(&data.payload.unwrap_or_default()) {


            match &event_data.get("Event-Name").and_then(|s| Some(s.as_str())) {

                Some("CHANNEL_CREATE") if event_data.get("Call-Direction").and_then(|s| Some(s.as_str())) == Some("inbound") => {
                    println!("[{}] <{}> new      [{}] {}", 
                             event_data.get("Event-Date-GMT").unwrap(),
                             event_data.get("Channel-Call-UUID").unwrap(),
                             event_data.get("Call-Direction").unwrap(),
                             event_data.get("Channel-Name").unwrap(),
                             );
                },

                Some("CHANNEL_OUTGOING") => {
                    println!("[{}] <{}> trying   [{}] {}", 
                             event_data.get("Event-Date-GMT").unwrap(),
                             event_data.get("Channel-Call-UUID").unwrap(),
                             event_data.get("Call-Direction").unwrap(),
                             event_data.get("Channel-Name").unwrap(),
                             );
                },

                Some("CHANNEL_ANSWER") => {
                    println!("[{}] <{}> answered [{}] {}", 
                             event_data.get("Event-Date-GMT").unwrap(),
                             event_data.get("Channel-Call-UUID").unwrap(),
                             event_data.get("Call-Direction").unwrap(),
                             event_data.get("Channel-Name").unwrap(),
                             );
                },

                Some("CHANNEL_BRIDGE") => {
                    println!("[{}] <{}> bridge   [{}] {}", 
                             event_data.get("Event-Date-GMT").unwrap(),
                             event_data.get("Channel-Call-UUID").unwrap(),
                             event_data.get("Call-Direction").unwrap(),
                             event_data.get("Channel-Name").unwrap(),
                             );
                },

                Some("CHANNEL_STATE") if event_data.get("Channel-State").and_then(|s| Some(s.as_str())) == Some("BRIDGE") => {
                     println!("[{}] <{}> bridge   [{}] {}      ---------", 
                             event_data.get("Event-Date-GMT").unwrap(),
                             event_data.get("Channel-Call-UUID").unwrap(),
                             event_data.get("Call-Direction").unwrap(),
                             event_data.get("Channel-Name").unwrap(),
                             );               
                },

                Some("CHANNEL_HANGUP_COMPLETE") => {
                    println!("[{}] <{}> hangup   [{}] {}", 
                             event_data.get("Event-Date-GMT").unwrap(),
                             event_data.get("Channel-Call-UUID").unwrap(),
                             event_data.get("Call-Direction").unwrap(),
                             event_data.get("Channel-Name").unwrap(),
                             );
                },

                Some(name) => {
                    //println!("Weird af name: {:?}", name);
                },

                _ => {}
            }
        }
*/
    }

    Ok(())
}
