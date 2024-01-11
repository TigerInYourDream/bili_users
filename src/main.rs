use log::info;
use reqwest::header::USER_AGENT;
use tokio::fs;

pub mod prepare;

use serde::Deserialize;
use std::collections::HashMap;

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

#[derive(Deserialize, Debug)]
struct Nameplate {
    nid: i32,
    name: String,
    image: String,
    image_small: String,
    level: String,
    condition: String,
}

#[derive(Deserialize, Debug)]
struct Official {
    role: i32,
    title: String,
    desc: String,
    #[serde(rename = "type")]
    type_: i32,
}

#[derive(Deserialize, Debug)]
struct Attestation {
    #[serde(rename = "type")]
    type_: i32,
    common_info: HashMap<String, String>,
    splice_info: HashMap<String, String>,
    icon: String,
    desc: String,
}

#[derive(Deserialize, Debug)]
struct ExpertInfo {
    title: String,
    state: i32,
    #[serde(rename = "type")]
    type_: i32,
    desc: String,
}

#[derive(Deserialize, Debug)]
struct Colour {
    dark: String,
    normal: String,
}

#[derive(Deserialize, Debug)]
struct Honours {
    mid: i32,
    colour: Colour,
    tags: Option<Vec<String>>,
    is_latest_100honour: i32,
}

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

#[derive(Deserialize, Debug)]
struct CardResponse {
    code: i32,
    msg: String,
    message: String,
    data: Vec<Data>,
}

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    prepare::init_log();

    let ua = fs::read("./source/user_agent.txt").await?;
    let ua = String::from_utf8(ua)?;

    let main = "https://api.vc.bilibili.com/account/v1/user/cards";
    let url = format!("{}?{}",main,"uids=1,2,3");
    info!("mian {:?}",url);

    let client = reqwest::Client::new();
    let r = client
    .get(url)
    .header(USER_AGENT,"Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36")
    .header("Cookie", "SESSDATA=xxxxx")
    .send()
    .await?
    .json::<CardResponse>()
    .await?;
    info!("{r:?}");
    // label_theme

    Ok(())
}
