use super::super::*;
use super::*;

#[derive(serde::Deserialize, Debug)]
pub struct Req {
    // #[serde(default)]
    pub tag: Option<String>,

    // #[serde(default)]
    pub author: Option<String>,

    // #[serde(default)]
    pub favorited: Option<String>,

    #[serde(default = "default_limit")]
    pub limit: i64,

    #[serde(default = "default_offset")]
    pub offset: i64,
}

#[derive(serde::Serialize)]
pub struct Res {
    articles: Vec<ResArticle>,

    #[serde(rename = "articlesCount")]
    articles_count: usize,
}

fn default_offset() -> i64 {
    0
}

fn default_limit() -> i64 {
    20
}

pub async fn handler(req: tide::Request<crate::State>) -> tide::Result {
    let query: Req = req.query()?;

    let db_pool = req.state().postgres_pool.clone();

    let payload = req.ext::<crate::middlewares::jwt_token::JWTPayload>();

    let article_entities = crate::applications::article::list(db_pool.clone(), query).await?;

    let res_articles = get_res_articles(article_entities, payload, db_pool).await?;

    let len = res_articles.len();

    response_ok_and_json(Res {
        articles: res_articles,
        articles_count: len,
    })
}
