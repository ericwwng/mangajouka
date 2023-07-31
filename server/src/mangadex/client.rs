use crate::mangadex::common::SupportedLanguage;
use anyhow::Result;
use bytes::Bytes;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

type LocalizedString = std::collections::HashMap<String, String>;

#[derive(Serialize, Deserialize, Debug)]
pub struct MangaList {
    pub data: Vec<Manga>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Manga {
    pub id: Uuid,
    pub r#type: String,
    pub attributes: MangaAttributes,
    #[serde(default)]
    pub relationships: Vec<Relationship>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MangaAttributes {
    pub title: LocalizedString,
    pub alt_titles: Vec<LocalizedString>,
    pub description: LocalizedString,
    pub is_locked: bool,
    pub tags: Vec<Tag>,
}

#[derive(Deserialize, Debug)]
struct Tags {
    data: Vec<Tag>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    id: Uuid,
    attributes: TagAttributes,
}

#[derive(Serialize, Deserialize, Debug)]
struct TagAttributes {
    name: LocalizedString,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Relationship {
    pub id: Uuid,
    pub r#type: String,
    #[serde(default)]
    // Always assume relationships are cover attributes since that is all we care about (very bad)
    pub attributes: CoverAttributes,
}

#[derive(Default, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CoverAttributes {
    pub file_name: String,
}

async fn send_request<T, U>(uri: &str, request_body: Option<U>) -> Result<T>
where
    T: DeserializeOwned,
    U: Serialize,
{
    let base_url = "https://api.mangadex.org";

    let full_url = format!("{}{}", base_url, uri);

    let client = reqwest::Client::new();
    let mut res = client.get(full_url);

    if let Some(body) = request_body {
        res = res.query(&body);
    }

    let res = res.send().await?;

    let out = &res.text().await?;

    let body_map: T = serde_json::from_str(&out)?;

    Ok(body_map)
}

pub async fn get_tag_ids(included_tags: &Vec<&str>, language: SupportedLanguage) -> Vec<String> {
    let body_map: Tags = send_request("/manga/tag", None::<()>).await.unwrap();

    let mut included_tag_ids: Vec<String> = Vec::new();

    for tag in body_map.data {
        let en_tag_name = tag
            .attributes
            .name
            .get(language.get_supported_language_string())
            .unwrap();

        for included_tag in included_tags {
            if en_tag_name.to_uppercase() == included_tag.to_uppercase() {
                included_tag_ids.push(tag.id.to_string());
            }
        }
    }

    included_tag_ids
}

pub async fn get_manga(included_tag_ids: &Vec<String>, limit: i32, offset: i32) -> MangaList {
    let request_content = [
        ("includedTags[]", included_tag_ids.join(",")),
        ("includes[]", "cover_art".to_string()),
        ("order[rating]", "desc".to_string()),
        ("limit", limit.to_string()),
        ("offset", offset.to_string()),
    ];

    let body_map: MangaList = send_request("/manga", Some(request_content)).await.unwrap();
    body_map
}

pub async fn get_cover_art_url(manga_id: &Uuid, filename: &str) -> String {
    let base_url = "https://uploads.mangadex.org";

    let full_url = format!("{}{}/{}/{}", base_url, "/covers", manga_id, filename);

    full_url
}

pub async fn get_cover_art(manga_id: &Uuid, filename: &str) -> Bytes {
    let base_url = "https://uploads.mangadex.org";

    let full_url = format!("{}{}/{}/{}", base_url, "/covers", manga_id, filename);

    let client = reqwest::Client::new();
    let res = client.get(full_url).send().await.unwrap();

    let bytes = &res.bytes().await.unwrap();
    bytes.clone()
}
