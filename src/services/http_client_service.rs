use actix_web::{client::{Client}, ResponseError};
use serde::{de::DeserializeOwned, Serialize};

#[warn(unused_must_use)]
#[allow(dead_code)]
pub async fn get<T: DeserializeOwned>(url: String) -> Result<T, ()> {
    let response = Client::default()
        .get(url.clone())
        .send()
        .await;

    let mut response = match response {
        Ok(response) => response,
        Err(err) => {
            error!("request url {}", &url);
            error!("Error send request {:?}", err);
            return Err(());
        }
    };

    let body = match response.body().limit(1_000_000).await {
        Ok(body) => body,
        Err(err) => {
            error!("Request Get Error parser body status code: {}", err.status_code());
            error!("request url {}", &url);
            return Err(());
        }
    };

    let body_data = match serde_json::from_slice::<T>(&body) {
        Ok(body_data) => body_data,
        Err(_err) => {
            return Err(());
        }
    };

    Ok(body_data)
}

#[warn(unused_must_use)]
#[allow(dead_code)]
pub async fn post<T: DeserializeOwned, P: Serialize>(url: &String, params: P) -> Result<T, ()> {
    let response = Client::default()
        .post(url)
        .send_json(&params)
        .await;
    let mut response = match response {
        Ok(response) => response,
        Err(err) => {
            error!("request url {}", &url);
            error!("Error send request {:?}", err);
            return Err(());
        }
    };

    let body = match response.body().limit(1_000_000).await {
        Ok(body) => body,
        Err(err) => {
            error!("Request Get Error parser body status code: {}", err.status_code());
            error!("request url {}", &url);
            return Err(());
        }
    };

    let body_data = match serde_json::from_slice::<T>(&body) {
        Ok(body_data) => body_data,
        Err(_err) => {
            return Err(());
        }
    };

    Ok(body_data)
}
