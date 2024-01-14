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

fn sign_(ts: u64, token: &str) -> String {
    let security = &format!("{}\n{}", ts, token);
    let hasher =
        HmacSha256::new_from_slice(security.as_bytes()).expect("HMAC can take key of any size");
    let result = hasher.finalize();
    let code_bytes = result.into_bytes();
    Base64::encode_string(&code_bytes)
}

impl BotMsg {
    fn new_msg(content: Content, token: &str) -> Self {
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        let sign = sign_(ts, token);
        Self {
            timestamp: ts.to_string(),
            msg_type: "text".to_string(),
            content,
            sign,
        }
    }
}
pub struct Bot<'a> {
    client: reqwest::Client,
    url: &'a str,
    token: &'a str,
}

impl<'a> Bot<'a> {
    pub fn new(url: &'a str, token: &'a str) -> Self {
        let client = reqwest::Client::new();
        Self { client, url, token }
    }

    pub async fn send(&self, content: &str) -> anyhow::Result<String> {
        let msg = BotMsg::new_msg(
            Content {
                text: content.to_string(),
            },
            self.token,
        );

        let response = self
            .client
            .post(self.url)
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
        let bot = Bot::new("url", "token");
        let text = bot.send("life time hello").await?;
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
        let sign = sign_(ts, "SECURITY");
        println!("{:?}", sign);
    }
}
