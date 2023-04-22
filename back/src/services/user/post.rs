use super::super::*;
use super::*;
use crate::utils::*;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Req {
    pub user: ReqUser,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ReqUser {
    #[serde(default)]
    pub username: String,

    #[serde(default)]
    pub email: String,

    #[serde(default)]
    pub password: String,

    pub image: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Res {
    user: ResAuthUser,
}

pub fn error_handler<'a>(
    mut req: tide::Request<crate::State>,
    next: tide::Next<'a, crate::State>,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = tide::Result> + Send + 'a>> {
    Box::pin(async {
        let req_body = req.body_json::<Req>().await?;

        let Req {
            user:
                ReqUser {
                    username,
                    email,
                    password,
                    ..
                },
        } = &req_body;

        let res = tide::Response::new(tide::StatusCode::UnprocessableEntity);

        let mut error_body = ErrorBody::default();

        if be_empty_string(&username) {
            error_body.errors.username = wrap_err_str("should not be blank");
            return set_error(res, error_body);
        }

        if be_empty_string(&email) {
            error_body.errors.email = wrap_err_str("should not be blank");
            return set_error(res, error_body);
        }

        if be_empty_string(&password) {
            error_body.errors.password = wrap_err_str("should not be blank");
            return set_error(res, error_body);
        }

        let db_pool = &req.state().postgres_pool;

        let mut have_errors = false;

        let (username, email) = futures::join!(
            crate::applications::user::have_with_username(db_pool, &username),
            crate::applications::user::have_with_email(db_pool, &email)
        );

        if username? {
            error_body.errors.username = wrap_err_str("has been already taken");
            have_errors = true;
        }

        if email? {
            error_body.errors.email = wrap_err_str("has been already taken");
            have_errors = true
        }

        if have_errors {
            return set_error(res, error_body);
        }

        req.set_ext(req_body);

        Ok(next.run(req).await)
    })
}

pub async fn handler(req: tide::Request<crate::State>) -> tide::Result {
    let Req { mut user } = req.ext::<Req>().unwrap().clone();

    let db_pool = &req.state().postgres_pool;

    // set default image url if None
    let default_avatar = std::env::var("DEFAULT_AVATAR")?;
    user.image = Some(String::from(default_avatar));

    // connect dao
    let user_entity = crate::applications::user::create(db_pool, &user).await?;

    let res_user = get_res_user(user_entity)?;

    response_ok_and_json(Res {
        user: res_user,
    })
}

#[cfg(test)]
pub mod tests {
    use tide::Server;

    use super::*;
    use crate::applications::user::tests::delete;
    use crate::server;
    use crate::services::user::tests::*;
    use crate::State;

    pub const URL: &str = "api/v1/users";

    async fn init_tumple() -> (Server<State>, Req) {
        let app = server().await.unwrap();

        let user = Req {
            user: ReqUser {
                username: String::from("someusernamenotconflit"),
                email: String::from("someemailnotconflit"),
                password: String::from("12342"),
                image: None,
            },
        };

        (app, user)
    }

    async fn delete_and_assert(db_pool: &sqlx::PgPool, email: String) {
        let result = delete(db_pool, &email).await;

        assert_eq!(result, true);
    }
    #[async_std::test]
    async fn test_post_success() {
        let (app, user) = init_tumple().await;
        // should register
        let res = request_post(user.clone(), URL, app.clone()).await;

        assert_eq!(res.borrow_mut().status(), tide::StatusCode::Ok);

        // 'res.username' and 'req.username' should be same
        let user_res: Res = res.borrow_mut().body_json().await.unwrap();

        assert_eq!(user.user.username, user_res.user.username);

        // 'res.image' should be default image url
        let default_avatar = std::env::var("DEFAULT_AVATAR").unwrap();

        assert_eq!(user_res.user.image, default_avatar);

        delete_and_assert(&app.state().postgres_pool, user_res.user.email).await;
    }

    #[async_std::test]
    async fn test_post_blank() {
        let (app, user) = init_tumple().await;
        // username should not be empty
        // priority 'username' > 'email'
        let mut user_test = user.clone();
        user_test.user.username = String::from(" ");
        user_test.user.email = String::from("    ");
        user_test.user.password = String::from("    ");

        let res = request_post(user_test, URL, app.clone()).await;

        assert_eq!(
            res.borrow_mut().status(),
            tide::StatusCode::UnprocessableEntity
        );
        let error: ErrorBody = res.borrow_mut().body_json().await.unwrap();
        assert_eq!(error.errors.username.unwrap()[0], "should not be blank");

        // email should not be empty
        // priority 'email' > 'password'
        let mut user_test = user.clone();
        user_test.user.email = String::from("");
        user_test.user.password = String::from("     ");
        let res = request_post(user_test, URL, app.clone()).await;
        assert_eq!(
            res.borrow_mut().status(),
            tide::StatusCode::UnprocessableEntity
        );
        let error: ErrorBody = res.borrow_mut().body_json().await.unwrap();
        assert_eq!(error.errors.email.unwrap()[0], "should not be blank");

        // password should not be empty
        let mut user_test = user;
        user_test.user.password = String::from("    ");
        let res = request_post(user_test, URL, app).await;
        assert_eq!(res.borrow().status(), tide::StatusCode::UnprocessableEntity);
        let error: ErrorBody = res.borrow_mut().body_json().await.unwrap();
        assert_eq!(error.errors.password.unwrap()[0], "should not be blank");
    }

    #[async_std::test]
    async fn test_post_exists() {
        let (app, user) = init_tumple().await;

        // should register
        let res = request_post(user.clone(), URL, app.clone()).await;

        assert_eq!(res.borrow_mut().status(), tide::StatusCode::Ok);

        // should register failed. email and user exists
        let res = request_post(user.clone(), URL, app.clone()).await;

        assert_eq!(
            res.borrow_mut().status(),
            tide::StatusCode::UnprocessableEntity
        );

        let error: ErrorBody = res.borrow_mut().body_json().await.unwrap();

        assert_eq!(error.errors.email.unwrap()[0], "has been already taken");

        assert_eq!(error.errors.username.unwrap()[0], "has been already taken");

        // should be blank error
        // priority 'error.blank' > 'error.exists'
        let mut user_test = user.clone();
        user_test.user.password = String::from("   ");

        let res = request_post(user_test.clone(), URL, app.clone()).await;

        assert_eq!(
            res.borrow_mut().status(),
            tide::StatusCode::UnprocessableEntity
        );

        let error: ErrorBody = res.borrow_mut().body_json().await.unwrap();

        assert_eq!(error.errors.password.unwrap()[0], "should not be blank");

        delete_and_assert(&app.state().postgres_pool, user.user.email).await;
    }
}
