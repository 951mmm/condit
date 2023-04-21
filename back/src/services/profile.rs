use super::*;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Res {
    pub profile: ResProfile,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Default)]
pub struct ResProfile {
    pub username: String,

    pub bio: Option<String>,

    pub image: String,

    pub following: bool,
}

pub fn get_follower_and_followee(
    req: &tide::Request<crate::State>,
) -> tide::Result<(String, String)> {
    let followee = req.param("username")?;

    let payload = req.ext::<crate::middlewares::jwt_token::JWTPayload>().unwrap();

    let crate::middlewares::jwt_token::JWTPayload {
        username: follower, ..
    } = payload;

    Ok((String::from(follower), String::from(followee)))
}

pub mod get {

    use super::*;

    pub async fn handler(req: tide::Request<crate::State>) -> tide::Result {
        let followee = req.param("username")?;

        let followee = String::from(followee);

        let db_pool = req.state().postgres_pool.clone();

        let profile = match req.ext::<crate::middlewares::jwt_token::JWTPayload>() {
            Some(payload) => {
                let crate::middlewares::jwt_token::JWTPayload {
                    username: follower, ..
                } = payload;

                let follower = String::from(follower);

                crate::applications::profile::get(db_pool, follower, followee).await?
            },
            None => crate::applications::profile::get_without_auth(db_pool, followee).await?
        };

        response_ok_and_json(Res { profile })
    }
}

pub mod post {

    use super::*;

    pub async fn handler(req: tide::Request<crate::State>) -> tide::Result {
        let (follower, followee) = get_follower_and_followee(&req)?;

        let db_pool = req.state().postgres_pool.clone();

        // build follow relationship
        match crate::applications::profile::follow(
            db_pool.clone(),
            follower.clone(),
            followee.clone(),
        )
        .await
        {
            Ok(_) => {}
            Err(err) => {
                let mut res = tide::Response::new(tide::StatusCode::InternalServerError);
                res.set_error(err);
                return Ok(res);
            }
        };

        // set res body with 'profile'
        let profile = crate::applications::profile::get(db_pool, follower, followee).await?;

        response_ok_and_json(Res { profile })
    }
}

pub mod delete {
    use super::*;

    pub async fn handler(req: tide::Request<crate::State>) -> tide::Result {
        let (follower, followee) = get_follower_and_followee(&req)?;

        let db_pool = req.state().postgres_pool.clone();

        match crate::applications::profile::unfollow(
            db_pool.clone(),
            follower.clone(),
            followee.clone(),
        )
        .await
        {
            Ok(_) => {}
            Err(err) => {
                let mut res = tide::Response::new(tide::StatusCode::InternalServerError);
                res.set_error(err);
                return Ok(res);
            }
        }

        let profile =
            crate::applications::profile::get(db_pool, follower.clone(), followee.clone()).await?;

        response_ok_and_json(Res { profile })
    }
}
