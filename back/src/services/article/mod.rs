pub mod list;

#[derive(serde::Deserialize, serde::Serialize, Debug,  Default)]
pub struct ResArticle {
    pub slug: String,

    pub title: String,

    pub description: String,

    pub body: String,

    // #[serde(rename = "tagList")]
    // pub tag_list: Vec<String>,

    #[serde(rename = "createdAt")]
    pub created_at: String,

    #[serde(rename = "updatedAt")]
    pub updated_at: String,

    pub author: crate::services::profile::ResProfile
}