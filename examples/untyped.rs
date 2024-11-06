use compact_str::CompactString;
use std::collections::HashMap;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio_util::codec::{FramedRead};
use futures::stream::StreamExt;

use fs_esl_codec::codec::EslCodec;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    // Open socket
    let listen = std::env::args().nth(1).expect("Missing LISTEN");
    let mut stream = TcpStream::connect(&listen).await.unwrap();

    // Authorise against an ESL
    let message = b"auth ClueCon\n\n";
    stream.write_all(message).await.unwrap();

    // Subscribe to all types of events in json format
    let message = b"event json ALL\n\n";
    stream.write_all(message).await.unwrap();

    // Instantiate ESL parser
    let in_codec = EslCodec::new();

    // Create tokio framedreader using EslCodec
    let mut framed_read = FramedRead::new(stream, in_codec);

    // Read freeswitch-emitted messages one-by-one
    while let Some(Ok(data)) = framed_read.next().await {

        // Decode into a hashmap of string key-value pairs
        if let Ok(event_data) = serde_json::from_str::<HashMap<CompactString,CompactString>>(&data.payload.unwrap_or_default()) {

            match &event_data.get("Event-Name").and_then(|s| Some(s.as_str())) {

                // First inbound INVITE received
                Some("CHANNEL_CREATE") if event_data.get("Call-Direction").and_then(|s| Some(s.as_str())) == Some("inbound") => {
                    println!("[{}] <{}> new      [{}] {}", 
                             event_data.get("Event-Date-GMT").unwrap(),
                             event_data.get("Channel-Call-UUID").unwrap(),
                             event_data.get("Call-Direction").unwrap(),
                             event_data.get("Channel-Name").unwrap(),
                             );
                },

                // leg b originated
                Some("CHANNEL_OUTGOING") => {
                    println!("[{}] <{}> trying   [{}] {}", 
                             event_data.get("Event-Date-GMT").unwrap(),
                             event_data.get("Channel-Call-UUID").unwrap(),
                             event_data.get("Call-Direction").unwrap(),
                             event_data.get("Channel-Name").unwrap(),
                             );
                },

                // leg b answered
                Some("CHANNEL_ANSWER") => {
                    println!("[{}] <{}> answered [{}] {}", 
                             event_data.get("Event-Date-GMT").unwrap(),
                             event_data.get("Channel-Call-UUID").unwrap(),
                             event_data.get("Call-Direction").unwrap(),
                             event_data.get("Channel-Name").unwrap(),
                             );
                },

                // Leg a bridged to leg b
                Some("CHANNEL_BRIDGE") => {
                    println!("[{}] <{}> bridge   [{}] {}", 
                             event_data.get("Event-Date-GMT").unwrap(),
                             event_data.get("Channel-Call-UUID").unwrap(),
                             event_data.get("Call-Direction").unwrap(),
                             event_data.get("Channel-Name").unwrap(),
                             );
                },

                // Leg b hangup
                Some("CHANNEL_HANGUP_COMPLETE") => {
                    println!("[{}] <{}> hangup   [{}] {}", 
                             event_data.get("Event-Date-GMT").unwrap(),
                             event_data.get("Channel-Call-UUID").unwrap(),
                             event_data.get("Call-Direction").unwrap(),
                             event_data.get("Channel-Name").unwrap(),
                             );
                },

                _ => {}
            }
        }
    }

    Ok(())
}
