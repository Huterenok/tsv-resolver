//TODO

use anyhow::{anyhow, Context, Ok};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use url::Url;

const TWITCH_GQL: &str = "https://gql.twitch.tv/gql";
const CLIENT_ID: &str = "kimne78kx3ncx6brgo4mv6wki5h1ko";

pub async fn fetch(vod_url: Url) -> anyhow::Result<String> {
    if !vod_url
        .domain()
        .context("Invalid twitch url")?
        .ends_with("twitch.tv")
    {
        return Err(anyhow!("Invalid domain: must be twitch.tv"));
    }

    let vod_id = vod_url
        .path_segments()
        .context("Invalid segments: must be like /videos/12345")?
        .last()
        .context("No vod id found in path segments")?
        .parse::<usize>()
        .context("Invalid vod id")?;

    let client = reqwest::Client::new();
    let TwitchResponse {
        data,
        extensions: _,
    } = fetch_twitch_data_gql(vod_id, &client).await?;

    let base_m3u8_url = format!(
        "{}/{}/chunked/",
        data.video
            .seek_previews_url
            .host_str()
            .context("Wrong twitch response")?,
        data.video
            .seek_previews_url
            .path_segments()
            .context("Wrong twitch response")?
            .next()
            .context("Wrong twitch response")?
    );

    let m3u8_data = fetch_m3u8(
        format!(
            "{}://{}index-dvr.m3u8",
            data.video.seek_previews_url.scheme(),
            base_m3u8_url
        ),
        &client,
    )
    .await?;

    //TODO
    Ok(m3u8_data
        .lines()
        .map(|l| {
            if l.contains(".ts") {
                format!("{}{}\n", base_m3u8_url, l.replace("unmuted", "muted"))
            } else {
                format!("{}\n", l)
            }
        })
        .collect())
}

#[derive(Deserialize, Serialize, Clone)]
pub struct TwitchResponse {
    data: VodData,
    extensions: VodExtensionsData,
}

#[derive(Deserialize, Serialize, Clone)]
struct VodExtensionsData {
    #[serde(rename = "durationMilliseconds")]
    duration_milliseconds: usize,
    #[serde(rename = "requestID")]
    request_id: String,
}

#[derive(Deserialize, Serialize, Clone)]
struct VodData {
    video: VodVideoData,
}

#[derive(Deserialize, Serialize, Clone)]
struct VodVideoData {
    #[serde(rename = "broadcastType")]
    broadcast_type: BroadcastType,
    #[serde(rename = "createdAt")]
    created_at: String,
    #[serde(rename = "seekPreviewsURL")]
    seek_previews_url: Url,
    owner: VodOwnerData,
}
#[derive(Deserialize, Serialize, Clone)]
struct VodOwnerData {
    login: String,
}

#[derive(Deserialize, Serialize, Clone)]
enum BroadcastType {
    ARCHIVE,
    HIGHLIGHT,
}

pub async fn fetch_m3u8(url: String, client: &Client) -> anyhow::Result<String> {
    let m3u8_text = client
        .get(url)
        .send()
        .await
        .context("Failed to fetch m3u8 data")?
        .text()
        .await
        .context("Failed to transform to text m3u8 data")?;

    Ok(m3u8_text)
}

pub async fn fetch_twitch_data_gql(
    vod_id: usize,
    client: &Client,
) -> anyhow::Result<TwitchResponse> {
    let gql_req = generate_gql_req(vod_id)?;

    let response_bytes = client
        .post(TWITCH_GQL)
        .header("Client-Id", CLIENT_ID)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .body(gql_req)
        .send()
        .await
        .context("Failed to fetch twitch data")?
        .bytes()
        .await
        .context("Failed to transform to bytes twitch data")?;

    let twitch_response: TwitchResponse =
        serde_json::from_slice(&response_bytes).context("Invalid twitch response")?;
    Ok(twitch_response)
}

#[derive(Serialize)]
struct GqlReq {
    query: String,
}

pub fn generate_gql_req(vod_id: usize) -> anyhow::Result<String> {
    serde_json::to_string(&GqlReq {
    query: format!("query {{ video(id: \"{}\") {{ broadcastType, createdAt, seekPreviewsURL, owner {{ login }} }} }}", vod_id)
}).context("Failed to serialize GqlReq")
}
