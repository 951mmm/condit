use super::super::*;

pub async fn handler(req: tide::Request<crate::State>) -> tide::Result {
    let slug = req.param("slug")?;

    let article_id = String::from(slug);

    let db_pool = req.state().postgres_pool.clone();

    let article_id = string_to_uuid(&article_id)?;

    crate::applications::article::delete(db_pool.clone(), article_id).await?;

    if !cfg!(feature = "foreign_key_constraint") {
        crate::applications::tag::delete(db_pool.clone(), article_id).await?;

        crate::applications::favorite::delete_by_article(db_pool, article_id).await?;
    }

    Ok(tide::StatusCode::Ok.into())
}
