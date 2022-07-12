use reqwest::Client;

use crate::{
    constants::{Response, API_ROOT},
    sign_in::User,
};

#[derive(Debug)]
pub struct Finary {
    client: Client,
}

impl Finary {
    pub fn new(client: Client) -> Self {
        Finary { client }
    }

    pub async fn me(&self) -> User {
        let val: Response<User> = self
            .client
            .get(format!("{}/users/me", API_ROOT))
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        val.result.unwrap()
    }
}
