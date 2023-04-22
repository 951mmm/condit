use super::super::*;
use super::*;
use crate::utils::*;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Req {
    pub user: ReqUser,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ReqUser {
    pub email: String,

    pub password: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Res {
    pub user: ResAuthUser,
}

pub fn error_handler<'a>(
    mut req: tide::Request<crate::State>,
    next: tide::Next<'a, crate::State>,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = tide::Result> + Send + 'a>> {
    Box::pin(async {
        let req_body = req.body_json::<Req>().await?;

        let Req {
            user: ReqUser { email, password },
        } = &req_body;

        let res = tide::Response::new(tide::StatusCode::UnprocessableEntity);

        let mut error_body = ErrorBody::default();

        if be_empty_string(email) {
            error_body.errors.email = wrap_err_str("should not be blank");
            return set_error(res, error_body);
        }

        if be_empty_string(password) {
            error_body.errors.password = wrap_err_str("should not be blank");
            return set_error(res, error_body);
        }

        req.set_ext(req_body);
        Ok(next.run(req).await)
    })
}

pub async fn handler(req: tide::Request<crate::State>) -> tide::Result {
    let Req { user } = req.ext::<Req>().unwrap().clone();

    let db_pool = &req.state().postgres_pool;

    let user = crate::applications::user::have(db_pool, &user).await?;

    match user {
        Some(user) => {
            let crate::applications::user::Entity {
                username,
                image,
                email,
                bio,
                id,
                ..
            } = user;

            let token = gen_token(username.clone(), id)?;

            response_ok_and_json(Res {
                user: ResAuthUser {
                    username,
                    image,
                    email,
                    bio,
                    token,
                },
            })
        }
        None => {
            let res = tide::Response::new(tide::StatusCode::Forbidden);

            let mut error_body = ErrorBody::default();

            error_body.errors.email_or_password = wrap_err_str("is invalid");

            set_error(res, error_body)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;
    use crate::applications::user::tests::delete;
    use crate::server;
    use crate::services::user::tests::request_post;

    const URL: &str = "api/v1/users/login";

    #[async_std::test]
    async fn test_login_success_and_failed() {
        let app = server().await.unwrap();
        let email = String::from("somebody@some.com");
        let password = String::from("somebody");
        let username = String::from("somebody");

        let user = Req {
            user: ReqUser {
                email: email.clone(),
                password: password.clone(),
            },
        };

        let res = request_post(user.clone(), URL, app.clone()).await;

        let app_inner = app.clone();

        let status = res.borrow().status();

        match status {
            // should not exist
            tide::StatusCode::Forbidden => {
                let error: ErrorBody = res.borrow_mut().body_json().await.unwrap();

                assert_eq!(error.errors.email_or_password.unwrap()[0], "is invalid");
                let user = post::Req {
                    user: post::ReqUser {
                        email: email.clone(),
                        password: password.clone(),
                        username: username.clone(),
                        image: None,
                    },
                };

                // should create
                let res = request_post(user, post::tests::URL, app_inner).await;

                assert_eq!(res.borrow().status(), tide::StatusCode::Ok);
            }
            // should registered
            tide::StatusCode::Ok => {}
            _ => {
                panic!("error status code");
            }
        }

        // email should not be empty
        let mut user_test = user.clone();
        user_test.user.email = String::from(" ");

        let res = request_post(user_test, URL, app.clone()).await;

        assert_eq!(res.borrow().status(), tide::StatusCode::UnprocessableEntity);

        let error: ErrorBody = res.borrow_mut().body_json().await.unwrap();

        assert_eq!(error.errors.email.unwrap()[0], "should not be blank");

        let mut user_test = user.clone();
        user_test.user.password = String::from("     ");

        let res = request_post(user_test, URL, app.clone()).await;

        assert_eq!(res.borrow().status(), tide::StatusCode::UnprocessableEntity);

        let error: ErrorBody = res.borrow_mut().body_json().await.unwrap();

        assert_eq!(error.errors.password.unwrap()[0], "should not be blank");

        // should exist
        let res = request_post(user, URL, app.clone()).await;

        assert_eq!(res.borrow().status(), tide::StatusCode::Ok);

        let user_res: Res = res.borrow_mut().body_json().await.unwrap();

        let result = delete(&app.state().postgres_pool, &user_res.user.email).await;
        assert_eq!(result, true);
    }
}
