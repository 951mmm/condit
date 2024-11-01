use super::super::*;
use super::*;

#[derive(serde::Deserialize, Debug)]
pub struct Req {
    #[serde(default)]
    pub tag: String,

    #[serde(default)]
    pub author: String,

    #[serde(default)]
    pub favorited: String,

    #[serde(default = "default_limit")]
    pub limit: i64,

    #[serde(default = "default_offset")]
    pub offset: i64,
}

#[derive(serde::Serialize)]
pub struct Res {
    articles: Vec<ResArticle>,

    #[serde(rename = "articlesCount")]
    articles_count: i64,
}

fn default_offset() -> i64 {
    0
}

fn default_limit() -> i64 {
    20
}

pub async fn handler(req: tide::Request<crate::State>) -> tide::Result {
    let query: Req = req.query()?;

    let db_pool = &req.state().postgres_pool;

    let payload = req.ext::<crate::middlewares::jwt_token::JWTPayload>();

    let (article_entities, len) = crate::applications::article::list(db_pool, &query).await?;

    let res_articles = get_res_articles(article_entities, payload, db_pool).await?;

    tide::log::info!("resolved articles are: {:?}", res_articles);


    response_ok_and_json(Res {
        articles: res_articles,
        articles_count: len,
    })
}
