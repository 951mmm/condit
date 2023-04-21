pub async fn get_favorited(
    db_pool: sqlx::PgPool,
    article_id: uuid::Uuid,
    follower_id: uuid::Uuid,
) -> tide::Result<bool> {
    #[derive(serde::Deserialize)]
    struct FavoritedView {
        favorited: bool,
    }
    tide::log::info!(
        "article: {}, follower: {}",
        article_id.to_string(),
        follower_id.to_string()
    );
    let row = sqlx::query_as_unchecked!(
        FavoritedView,
        r#"
        select count(*)=1 as favorited from favoriting
        where article_id=$1 and follower_id=$2;
        "#,
        article_id,
        follower_id
    )
    .fetch_one(&db_pool)
    .await?;

    Ok(row.favorited)
}

pub async fn get_favorites_count(
    db_pool: sqlx::PgPool,
    article_id: uuid::Uuid,
) -> tide::Result<i64> {
    #[derive(serde::Deserialize)]
    struct FavoritesCountView {
        count: i64,
    }
    let row = sqlx::query_as_unchecked!(
        FavoritesCountView,
        r#"
        select count(*) from article
        inner join favoriting on article.id=favoriting.article_id
        and article.id=$1;
        "#,
        article_id
    )
    .fetch_one(&db_pool)
    .await?;
    Ok(row.count)
}
