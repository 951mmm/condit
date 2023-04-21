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
    pub is_optional: bool
}

impl Ware {
    pub fn new(key: String, is_optional: bool) -> tide::Result<Self> {
        let key = jsonwebtoken::DecodingKey::from_secret(&key.as_bytes());
        tide::log::info!("generate decodeing key");
        Ok(Self {
            base64_key: key,
            is_optional
        })
    }

}

#[tide::utils::async_trait]
impl<State: Clone + Send + Sync + 'static> tide::Middleware<State> for Ware {
    async fn handle(
        &self,
        mut req: tide::Request<State>,
        next: tide::Next<'_, State>,
    ) -> tide::Result {

        let res_unauth = tide::Response::new(tide::StatusCode::Unauthorized);

        if let Some(auth) = req.header("Authorization") {
            let values: Vec<_> = auth.into_iter().collect();

            // search "Token ..."
            for value in values {
                let value = value.as_str();
                if !value.starts_with("Token") {
                    continue;
                }

                // slice token out
                let token = &value["Token ".len()..];

                #[cfg(feature = "token_debug")]
                tide::log::info!("token is {}", token);

                // decrypt payload fron token
                let payload = match jsonwebtoken::decode::<JWTPayload>(
                    token,
                    &self.base64_key,
                    &jsonwebtoken::Validation::default(),
                ) {
                    Ok(payload) => payload,
                    Err(_) => match self.is_optional {
                        true => return Ok(next.run(req).await),
                        false => return Ok(res_unauth),
                    }
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

                #[cfg(feature = "token_debug")]
                tide::log::info!("current: {}, exp: {}", current_time, exp);

                // whether expired
                if current_time < exp {
                    req.set_ext(claims);
                    return Ok(next.run(req).await);
                } else {
                    match self.is_optional {
                        true => return Ok(next.run(req).await),
                        false => return Ok(res_unauth),
                    }
                }
            }
        }

        match self.is_optional {
            true => return Ok(next.run(req).await),
            false => return Ok(res_unauth),
        }
    }
}
