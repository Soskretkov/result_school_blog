use super::URL;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::json;
// use leptos::*;

pub async fn add_user<T>(new_user: &T)
where
    T: DeserializeOwned + Serialize,
{
    let url = format!("{URL}/users");
    // logging::log!("{} (add_user)", url);
    let client = reqwest::Client::new();

    client
        .post(url)
        .json(new_user) // установка тела запроса
        .send()
        .await
        .unwrap();
}

pub async fn update_user_field<T>(user_id: &str, field_name: &str, field_value: &T)
where
    T: DeserializeOwned + Serialize,
{
    let url = format!("{URL}/users/{}", user_id);
    let client = reqwest::Client::new();

    client
        .patch(url)
        .json(&json!({ field_name: field_value }))
        .send()
        .await
        .unwrap();
}
