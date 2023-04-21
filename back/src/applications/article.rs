//! there should be a func to join table article and user
//! there should be a func to join favorited and favoritedCunt

use super::*;

#[derive(serde::Deserialize, serde::Serialize, Debug, Default, sqlx::FromRow)]
pub struct Entity {
    pub id: uuid::Uuid,

    pub title: String,

    pub description: String,

    pub body: String,

    pub created_at: chrono::NaiveDateTime,

    pub updated_at: chrono::NaiveDateTime,

    pub author_id: uuid::Uuid,
}

pub async fn list(
    db_pool: sqlx::PgPool,
    query: crate::services::article::list::Req,
) -> tide::Result<Vec<Entity>> {
    let crate::services::article::list::Req {
        author,
        tag,
        favorited,
        limit,
        offset,
    } = &query;

    let sql_string = format!(
        r#"
        {}
        select
            article.id as id,
            title,
            description,
            body,
            created_at,
            updated_at,
            author_id
        from article
        inner join condituser on 
            author_id=condituser.id
            {}
        inner join tag on 
            tag.article_id=article.id
            {}
            {}
        group by article.id
        order by updated_at desc
        limit {} offset {};
        "#,
        empty_or_statement(
            r#"
            with
            favoriting_name as (
            select 
                username as follower_name, 
                article_id 
            from favoriting
            inner join condituser on favoriting.follower_id=condituser.id
        )"#,
            favorited
        ),
        empty_or_expr("and condituser.username=", author),
        empty_or_expr("and tag.name=", tag),
        empty_or_expr(
            r#"
            inner join favoriting_name on 
            favoriting_name.article_id=article.id
            and favoriting_name.follower_name=
            "#,
            favorited
        ),
        limit,
        offset,
    );

    tide::log::info!("sql string is: {}", sql_string);

    let row = sqlx::query_as(&sql_string).fetch_all(&db_pool).await?;
    Ok(row)
}

pub async fn list_feed(
    db_pool: sqlx::PgPool,
    follower_id: uuid::Uuid,
) -> tide::Result<Vec<Entity>> {
    let rows = sqlx::query_as!(
        Entity,
        r#"
        select 
            id,
            title,
            description,
            body,
            created_at,
            updated_at,
            author_id
        from article
        inner join favoriting
        on favoriting.article_id=article.id
        and favoriting.follower_id=$1
        "#,
        follower_id
    )
    .fetch_all(&db_pool)
    .await?;

    Ok(rows)
}

pub async fn get(db_pool: sqlx::PgPool, id: uuid::Uuid) -> tide::Result<Entity> {
    let row = sqlx::query_as!(
        Entity,
        r#"
        select * from article
        where article.id=$1;
        "#,
        id
    )
    .fetch_one(&db_pool)
    .await?;

    Ok(row)
}

pub async fn create(
    db_pool: sqlx::PgPool,
    req_article: crate::services::article::ReqWriteArticle,
    author_id: uuid::Uuid,
) -> tide::Result<Entity> {
    let row = sqlx::query_as!(
        Entity,
        r#"
        insert into article (title, description, body, author_id)
        values ($1, $2, $3, $4) returning *;
        "#,
        req_article.title,
        req_article.description,
        req_article.body,
        author_id
    )
    .fetch_one(&db_pool)
    .await?;

    Ok(row)
}

pub async fn update(
    db_pool: sqlx::PgPool,
    req_article: crate::services::article::ReqWriteArticle,
    author_id: uuid::Uuid,
) -> tide::Result<Entity> {
    let crate::services::article::ReqWriteArticle {
        title,
        description,
        body,
        ..
    } = req_article;

    let params = Joiner::build(",", be_empty_string)
        .join(empty_or_expr("title=", &title))
        .join(empty_or_expr("description=", &description))
        .join(empty_or_expr("body=", &body))
        .builder();

    let sql_string = format!(
        r#"
        update article
        set {}
        where id='{}'
        returning *;
        "#,
        params, author_id
    );

    let row = sqlx::query_as(&sql_string.as_str())
        .fetch_one(&db_pool)
        .await?;

    Ok(row)
}

pub async fn delete(db_pool: sqlx::PgPool, article_id: uuid::Uuid) -> tide::Result<()> {
    match sqlx::query!(
        r#"
        delete from article
        where id=$1;
        "#,
        article_id
    )
    .execute(&db_pool)
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into()),
    }
}
