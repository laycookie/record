use serde_json::{json, Value};
use crate::discord::websocketconfig::discord_intents;

pub struct Payload
{
    pub identify: Value,
    pub heart_beat: Value,
}

impl Payload
{
    pub fn build(intents: i32, token: &str) -> Self
    {
        let identify = json!(
            {
                "op": 2,
                "d": {
                "token": token,
                "properties": {
                    "$os": "windows",
                    "$browser": "record_user",
                    "$device": "record_user"
                },
                "intents": intents
            }
        });

        let heart_beat = json!(
            {
                "op": 1,
                "d": null
            }
        );

        Self { identify, heart_beat }
    }

}



