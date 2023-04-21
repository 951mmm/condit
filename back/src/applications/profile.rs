pub async fn get(
    db_pool: sqlx::PgPool,
    follower: String,
    followee: String,
) -> tide::Result<crate::services::profile::ResProfile> {
    // count(*)=1 should never be none
    let row = sqlx::query_as_unchecked!(
        crate::services::profile::ResProfile,
        r#"
        with 
        followee as (
            select followee_id from condituser 
            inner join following on condituser.id=following.follower_id where username=$1
        ),
        subscribe as (
            select count(*)=1 as following from followee 
            inner join condituser on followee.followee_id=condituser.id where username=$2
        ),
        profile as (
            select username, bio, image, following from condituser
            cross join subscribe where username=$1
        )
        select * from profile;
        "#,
        follower,
        followee,
    )
    .fetch_one(&db_pool)
    .await?;

    Ok(row)
}

pub async fn get_with_id(
    db_pool: sqlx::PgPool,
    follower_id: uuid::Uuid,
    followee_id: uuid::Uuid,
) -> tide::Result<crate::services::profile::ResProfile> {
    // count(*)=1 should never be none
    let row = sqlx::query_as_unchecked!(
        crate::services::profile::ResProfile,
        r#"
        with 
        followee as (
            select followee_id from condituser 
            inner join following on condituser.id=following.follower_id where condituser.id=$1
        ),
        subscribe as (
            select count(*)=1 as following from followee 
            inner join condituser on followee.followee_id=condituser.id where condituser.id=$2
        ),
        profile as (
            select username, bio, image, following from condituser
            cross join subscribe where condituser.id=$1
        )
        select * from profile;
        "#,
        follower_id,
        followee_id,
    )
    .fetch_one(&db_pool)
    .await?;

    Ok(row)
}


pub async fn get_with_id_without_auth(
    db_pool: sqlx::PgPool,
    followee_id: uuid::Uuid,
) -> tide::Result<crate::services::profile::ResProfile> {
    let row = sqlx::query_as_unchecked!(
        crate::services::profile::ResProfile,
        r#"
        with 
        following_false as (
            select false as following
        )
        select 
            username,
            bio,
            image,
            following
        from condituser
        cross join following_false where id=$1;
        "#,
        followee_id
    )
    .fetch_one(&db_pool)
    .await?;

    Ok(row)
}

pub async fn get_without_auth(
    db_pool: sqlx::PgPool,
    followee: String,
) -> tide::Result<crate::services::profile::ResProfile> {
    let row = sqlx::query_as_unchecked!(
        crate::services::profile::ResProfile,
        r#"
        with following_false as (
            select false as following
        )
        select 
            username,
            bio,
            image,
            following
        from condituser
        cross join following_false where username=$1;
        "#,
        followee
    ).fetch_one(&db_pool)
    .await?;
    Ok(row)
}

pub async fn follow(db_pool: sqlx::PgPool, follower: String, followee: String) -> tide::Result<()> {
    match sqlx::query!(
        r#"
        with
        follower as (
            select id from condituser
            where username=$1
        ),
        followee as (
            select id from condituser
            where username=$2
        )
        insert into following(follower_id, followee_id) values ((select * from follower), (select * from followee))
        "#,
        follower,
        followee
    ).execute(&db_pool).await {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into()),
    }
}

pub async fn unfollow(
    db_pool: sqlx::PgPool,
    follower: String,
    followee: String,
) -> tide::Result<()> {
    match sqlx::query!(
        r#"
        with 
        follower as (
            select distinct follower_id from condituser
            inner join following on condituser.id=following.follower_id
            where username=$1
        ),
        followee as (
            select distinct followee_id from condituser
            inner join following on condituser.id=following.followee_id
            where username=$2
        )
        delete from following where follower_id=(select * from follower) and followee_id=(select * from followee);
        "#,
        follower,
        followee
    ).execute(&db_pool).await {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into()),
    }
}
