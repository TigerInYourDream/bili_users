use log::{error, info};
use rand::prelude::*;
use sqlx::{Connection, SqliteConnection};

use reqwest::header::USER_AGENT;
use tokio::fs;

pub mod prepare;

use serde::Deserialize;
use std::{collections::HashMap, time::Duration};

use crate::prepare::{BaseCol, insert};

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Label {
    path: String,
    text: String,
    label_theme: String,
    text_color: String,
    bg_style: i32,
    bg_color: String,
    border_color: String,
    use_img_label: bool,
    img_label_uri_hans: String,
    img_label_uri_hant: String,
    img_label_uri_hans_static: String,
    img_label_uri_hant_static: String,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Vip {
    #[serde(rename = "type")]
    type_: i32,
    status: i32,
    due_date: i64,
    vip_pay_type: i32,
    theme_type: i32,
    label: Label,
    avatar_subscript: i32,
    nickname_color: String,
    role: i32,
    avatar_subscript_url: String,
    tv_vip_status: i32,
    tv_vip_pay_type: i32,
    tv_due_date: i64,
    #[serde(skip)]
    avatar_icon: HashMap<String, Vec<String>>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Pendant {
    pid: i32,
    name: String,
    image: String,
    expire: i32,
    image_enhance: String,
    image_enhance_frame: String,
    n_pid: i32,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Nameplate {
    nid: i32,
    name: String,
    image: String,
    image_small: String,
    level: String,
    condition: String,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Official {
    role: i32,
    title: String,
    desc: String,
    #[serde(rename = "type")]
    type_: i32,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Attestation {
    #[serde(rename = "type")]
    type_: i32,
    common_info: HashMap<String, String>,
    splice_info: HashMap<String, String>,
    icon: String,
    desc: String,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct ExpertInfo {
    title: String,
    state: i32,
    #[serde(rename = "type")]
    type_: i32,
    desc: String,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Colour {
    dark: String,
    normal: String,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Honours {
    mid: i32,
    colour: Colour,
    tags: Option<Vec<String>>,
    is_latest_100honour: i32,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Data {
    mid: i32,
    name: String,
    sex: String,
    face: String,
    sign: String,
    rank: i32,
    level: i32,
    silence: i32,
    vip: Vip,
    pendant: Pendant,
    nameplate: Nameplate,
    official: Official,
    birthday: i64,
    is_fake_account: i32,
    is_deleted: i32,
    in_reg_audit: i32,
    face_nft: i32,
    face_nft_new: i32,
    is_senior_member: i32,
    digital_id: String,
    digital_type: i32,
    attestation: Attestation,
    expert_info: ExpertInfo,
    honours: Honours,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct CardResponse {
    code: i32,
    msg: String,
    message: String,
    data: Vec<Data>,
}

pub const BILI_MAX_CARDS: usize = 50;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    prepare::init_log();

    let ua = fs::read("./source/user_agent.txt").await?;
    let ua = String::from_utf8(ua)?;
    let uas: Vec<&str> = ua.split('\n').collect();
    let len = uas.len();

    let distr = rand::distributions::Uniform::new_inclusive(0, len - 1);
    let mut rng = thread_rng();

    let main = "https://api.vc.bilibili.com/account/v1/user/cards";

    let mut conn = SqliteConnection::connect("./source/userinfo_db.db").await?;

    let mut start_mid = 1;
    let mut err_times = 0;

    loop {
        let mut uids: String = String::with_capacity(100);
        for id in 0..BILI_MAX_CARDS {
            if id == BILI_MAX_CARDS - 1 {
                uids.push_str(&format!("{}", start_mid));
            } else {
                uids.push_str(&format!("{},", start_mid));
            }
            start_mid += 1;
        }

        let url = format!("{}?uids={}", main, uids);
        info!("url {:}", url);
        let random_hit: usize = rng.sample(distr);
        info!("uas {:}", uas[random_hit]);
        let client = reqwest::Client::new();
        let r = client
            .get(url.clone())
            .header(USER_AGENT, uas[random_hit])
            .header("Cookie", "SESSDATA=xxxxx")
            .send()
            .await?
            .json::<CardResponse>()
            .await?;

        let mut base_data = Vec::with_capacity(50);
        for data in &r.data {
            let lt = &data.vip.label.label_theme;
            let mid = data.mid;
            let name = &data.name;
            info!("{} {} {}", mid, name, lt);
            let col =  BaseCol { mid, lable_theme: lt.clone(), name: name.clone() };
            base_data.push(col);
        }

        insert(&mut conn, &base_data).await?;

        if r.data.len() < 42 {
            err_times += 1;
        }

        if err_times > 10 {
            error!(
                "err_times > 10 stop the program start_mid is {:?}",
                start_mid
            );
            break;
        }

        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    // label_theme

    Ok(())
}
