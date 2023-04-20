#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct JWTPayload {
    pub id: String,
    pub username: String,
    pub exp: i64,
}

pub fn crypt<Payload: serde::Serialize + serde::de::DeserializeOwned + Send + Sync + 'static>(
    key: String,
    payload: &Payload,
) -> Result<String, jsonwebtoken::errors::Error> {
    jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        payload,
        &jsonwebtoken::EncodingKey::from_secret(&key.as_bytes()),
    )
}

#[derive(Clone)]
pub struct Ware {
    pub base64_key: jsonwebtoken::DecodingKey,
    pub optional: bool,
}

impl Ware {
    pub fn new(key: String, optional: bool) -> tide::Result<Self> {
        let key = jsonwebtoken::DecodingKey::from_secret(&key.as_bytes());
        tide::log::info!("generate decodeing key");
        Ok(Self {
            base64_key: key,
            optional,
        })
    }

    /// optional return
    /// - self.optional is 'true', go ahead if unauth.
    /// - self.optional is 'false', return StatusCode 'Unauthorize'
    pub async fn optional_res<State: Clone + Send + Sync + 'static>(
        &self,
        req: tide::Request<State>,
        res: tide::Response,
        next: tide::Next<'_, State>,
    ) -> tide::Result {
        match self.optional {
            true => Ok(next.run(req).await),
            false => Ok(res),
        }
    }

    pub fn white_list(
        &self,
        url: tide::http::Url
    ) -> tide::Result<bool> {
        let set = regex::RegexSet::new(&[
            r"^/users/login$",
            r"^/users$",
            r"^/user$",
        ])?;

        let path = url.path();

        Ok(set.is_match(path))
    }

    pub fn optional_list(
        &self,
        url: tide::http::Url
    ) -> tide::Result<bool> {
        let set = regex::RegexSet::new(&[
            r"^/profiles/+*(?<!/follow)$",
        ])?;

        let path = url.path();

        Ok(set.is_match(path))
    }
}

#[tide::utils::async_trait]
impl<State: Clone + Send + Sync + 'static> tide::Middleware<State> for Ware {
    async fn handle(
        &self,
        mut req: tide::Request<State>,
        next: tide::Next<'_, State>,
    ) -> tide::Result {

        if self.white_list(req.url().clone())? {
            return Ok(next.run(req).await);
        }

        let res_unauth = tide::Response::new(tide::StatusCode::Unauthorized);

        if let Some(auth) = req.header("Authorization") {
            let values: Vec<_> = auth.into_iter().collect();

            tide::log::info!("token content: {:#?}", values);

            // search "Token ..."
            for value in values {
                let value = value.as_str();
                if !value.starts_with("Token") {
                    continue;
                }

                // slice token out
                let token = &value["Token ".len()..];

                tide::log::info!("token is {}", token);

                // decrypt payload fron token
                let payload = match jsonwebtoken::decode::<JWTPayload>(
                    token,
                    &self.base64_key,
                    &jsonwebtoken::Validation::default(),
                ) {
                    Ok(payload) => payload,
                    Err(_) => return self.optional_res(req, res_unauth, next).await,
                };

                // get expire of claims
                let jsonwebtoken::TokenData { claims, .. } = payload;

                let JWTPayload { exp, .. } = claims;

                let current_time = chrono::Utc::now().timestamp();

                // should not auth expire when testing
                if cfg!(test) || cfg!(feature = "debug") {
                    req.set_ext(claims);
                    return Ok(next.run(req).await);
                }

                tide::log::info!("current: {}, exp: {}", current_time, exp);

                // whether expired
                if current_time < exp {
                    req.set_ext(claims);
                    return Ok(next.run(req).await);
                } else {
                    return self.optional_res(req, res_unauth, next).await;
                }
            }
        }

        self.optional_res(req, res_unauth, next).await
    }
}
