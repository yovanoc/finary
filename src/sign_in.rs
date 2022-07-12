use reqwest::{Client, StatusCode};
use crate::{
    constants::{Response, API_ROOT},
    finary::Finary,
    structs::{SignIn, SignInResponse}, 
    error::FinaryError
};



pub async fn sign_in(email: &str, password: &str) -> Result<Finary, FinaryError> {
    let ua = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.0 Safari/605.1.15";
    let client = Client::builder()
        .cookie_store(true)
        .user_agent(ua)
        .build()
        .map_err(|_| FinaryError::ClientBuildError)?;

    let sign_in = SignIn {
        email: email.into(),
        password: password.into(),
        device_id: ua.into(),
    };

    let ok = serde_json::to_string(&sign_in).map_err(|_| FinaryError::ClientBuildError)?;

    let response = client
        .post(format!("{}/auth/signin", API_ROOT))
        .header("content-length", ok.len())
        // .header("X-Finary-Client-Id", "webapp")
        .json(&sign_in)
        .send()
        .await
        .map_err(|_| FinaryError::ClientHttpError)?;

    
    let status = response.status();
    match status {
        StatusCode::CREATED => {
            let _sign: Response<SignInResponse> = response.json().await.map_err(|_| FinaryError::SignInResponseError)?;
            // println!("{:#?}", sign);
            Ok(Finary::new(client))
        },
        _ => {
            println!("{}", status);
            Err(FinaryError::CodeError)
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        let email = "";
        let password = "";
        let finary = sign_in(email, password).await;

        // println!("{:#?}", finary);

        match finary {
            Ok(finary) => {
                let me = finary.me().await;
                println!("{:#?}", me);
            }
            Err(e) => {
                println!("{:#?}", e);
            }
        }

        assert_eq!(3, 4)
    }
}
