use base64ct::{Base64, Encoding};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Deserialize, Serialize, Debug)]
pub struct Content {
    pub text: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug)]
pub struct BotMsg {
    pub timestamp: String,
    pub sign: String,
    pub msg_type: String,
    pub content: Content,
}
pub type HmacSha256 = Hmac<Sha256>;

fn sign_(ts: u64) -> String {
    let security = &format!("{}\n{}", ts, "$SECURITY");
    let hasher =
        HmacSha256::new_from_slice(security.as_bytes()).expect("HMAC can take key of any size");
    let result = hasher.finalize();
    let code_bytes = result.into_bytes();
    Base64::encode_string(&code_bytes)
}

impl BotMsg {
    pub fn new_msg(content: Content) -> Self {
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        let sign = sign_(ts);
        Self {
            timestamp: ts.to_string(),
            msg_type: "text".to_string(),
            content,
            sign,
        }
    }
}
pub struct Bot {
    client: reqwest::Client,
}

impl Default for Bot {
    fn default() -> Self {
        Self::new()
    }
}

impl Bot {
    pub fn new() -> Self {
        let client = reqwest::Client::new();
        Self { client }
    }

    pub async fn send(&self, content: &str) -> anyhow::Result<String> {
        let msg = BotMsg::new_msg(Content {
            text: content.to_string(),
        });

        let url =
            "https://open.feishu.cn/open-apis/bot/v2/hook/xxxxxxxx";

        let response = self
            .client
            .post(url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .json(&msg)
            .send()
            .await?;
        let text = response.text().await?;
        Ok(text)
    }
}

#[cfg(test)]
mod test {

    use crate::larkbot::{sign_, Bot};
        use std::{error::Error, time::SystemTime};

    #[tokio::test]
    pub async fn test_bot() -> Result<(), Box<dyn Error>> {
        let bot = Bot::new();
        let text =  bot.send("hello").await?;
        println!("{:?}", text);
        Ok(())
    }

    #[test]
    pub fn test_sign() {
        let ts = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        println!("{:?}", ts);
        let sign = sign_(ts);
        println!("{:?}", sign);
    }
}
