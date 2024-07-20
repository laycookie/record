use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures::{SinkExt, StreamExt};
use serde_json::to_string_pretty;
use tokio::time::{Duration, sleep};
use crate::discord::rest_api::discord_endpoints::ApiEndpoints::DiscordGateway;
use crate::discord::websocket::websocketconfig::discord_intents;
use crate::discord::websocket::websocketconfig::websocketconfig::Payload;

pub async fn websocket_init(token: &str) {
    let (ws_stream, _) = connect_async(DiscordGateway.get_url()).await.expect("Failed to connect");
    println!("Connected to WebSocket server");
    let (mut write, mut read) = ws_stream.split();
    // NOTE: To put more intents, you can use "|" operator
    // Example: DIRECT_MESSAGES | GUILDS | GUILD_MEMBERS
    let Payload { identify, heart_beat } = Payload::build(discord_intents::DIRECT_MESSAGES, token);
    write.send(Message::Text(identify.to_string())).await.expect("Failed to send IDENTIFY");
    tokio::spawn(
        async move
        {
            //TODO: Replace hardcoded milliseconds with data fetched from websocket
            println!("initializing");
            sleep(Duration::from_millis(41250 / 5)).await;
            write.send(Message::Text(heart_beat.to_string())).await.expect("failed to send heartbeat to a server");
            loop
            {
                sleep(Duration::from_millis(41250)).await;
                write.send(Message::Text(heart_beat.to_string())).await.expect("failed to send heartbeat to a server");
                println!("Send heart beat");
            }
        }
    );
    while let Some(message) = read.next().await
    {
        match message {
            Ok(Message::Text(text)) => {
                let json_response: serde_json::Value = serde_json::from_str(text.as_str()).unwrap();
                println!("Received: {}", to_string_pretty(&json_response).unwrap());
            }
            Ok(Message::Binary(bin)) => println!("Received binary message"),
            Err(e) => eprintln!("WebSocket error: {}", e),
            _ => (),
        }
    }
}

