//! there should be a func to join table article and user
//! there should be a func to join favorited and favoritedCunt

#[derive(serde::Deserialize, serde::Serialize, Default, sqlx::FromRow)]
pub struct Entity {
    pub id: uuid::Uuid,

    pub title: String,

    pub description: String,

    pub body: String,

    pub created_at: chrono::NaiveDate,

    pub updated_at: chrono::NaiveDate,

    pub author_id: uuid::Uuid,
}

pub async fn list(
    db_pool: sqlx::PgPool,
    query: crate::services::article::list::Req,
) -> tide::Result<Vec<Entity>> {
    let sql_string = format!(
        r#"
        with 
        favoriting_name as (
            select 
                username as follower_name, 
                article_id 
            from favoriting
            inner join condituser on favoriting.follower_id=condituser.id
        )
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
        inner join favoriting_name on 
        favoriting_name.article_id=article.id
            {}
        group by article.id
        order by updated_at desc
        limit {} offset {};
        "#,
        match &query.author {
            Some(string) => format!("and condituser.username='{}'",string),
            None => String::default()
        },
        match &query.tag {
            Some(string) => format!("and tag.name='{}'",string),
            None => String::default()
        },
        match &query.favorited {
            Some(string) => format!("and favoriting_name.follower_name='{}'",string),
            None => String::default()
        },
        query.limit,
        query.offset,
    );

    tide::log::info!("sql string is: {}", sql_string);
    
    let row = sqlx::query_as(
        sql_string.as_str()
    )
    .fetch_all(&db_pool)
    .await?;
    Ok(row)
}

pub async fn get_favorited_and_count_without_auth(db_pool: sqlx::PgPool, article_id: uuid::Uuid) -> tide::Result< (bool, i64)> {
    #[derive(serde::Deserialize)]
    struct FavoritesCountView {
        count: i64
    }
    let row = sqlx::query_as_unchecked!(
        FavoritesCountView,
        r#"
        select count(*) from article
        inner join favoriting on article.id=favoriting.article_id
        and article.id=$1;
        "#,
        article_id
    ).fetch_one(&db_pool).await?;
    Ok((false, row.count))
} 