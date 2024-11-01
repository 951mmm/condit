use async_std::println;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct JWTPayload {
    pub id: String,
    pub username: String,
    pub exp: i64,
}

pub fn crypt<Payload: serde::Serialize + serde::de::DeserializeOwned>(
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

        #[cfg(feature = "no_jwt")]
        return Ok(next.run(req).await);

         // Unauthorized response
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
                let payload = jsonwebtoken::decode::<JWTPayload>(
                    token,
                    &self.base64_key,
                    &jsonwebtoken::Validation::default(),
                );
                match payload {
                    Ok(payload) => {
                        tide::log::info!("JWT token claims is {:?}", payload.claims);
                        req.set_ext(payload.claims);
                        return Ok(next.run(req).await);
                    },
                    Err(e) =>{
                        tide::log::info!("JWT decode failed: {}", e);
                        match self.is_optional {
                        true => return Ok(next.run(req).await),
                        false => return Ok(res_unauth)
                    }}
                };

            }
        }

        // If no Authorization header, check optional setting
        match self.is_optional {
            true => return Ok(next.run(req).await),
            false => return Ok(res_unauth),
        }
    }
}
