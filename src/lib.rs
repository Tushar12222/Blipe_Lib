use std::collections::HashMap;

use reqwest::header::CONTENT_TYPE;
use serde_derive::Deserialize;
#[allow(unused)]
#[derive(Deserialize, Debug)]
struct PollResponse {
    message: String,
    job: i32,
}

#[derive(Deserialize, Debug)]
struct PushResponse {
    success: String,
}

pub struct Consumer {
    url: String,
}

pub struct Producer {
    url: String,
}

impl Consumer {
    pub fn new(url: String) -> Self {
        let new_url = url + "/poll";
        Consumer { url: new_url }
    }

    pub async fn consume(&self) -> i32 {
        let client = reqwest::Client::builder().build();
        let response = client
            .unwrap()
            .get(self.url.to_owned())
            .header(CONTENT_TYPE, "application/json")
            .send()
            .await;

        let r = response.unwrap().text().await.unwrap();
        let json: PollResponse = serde_json::from_str(r.as_str()).unwrap();
        json.job
    }
}

impl Producer {
    pub fn new(url: String) -> Self {
        let new_url = url + "/push";
        Producer { url: new_url }
    }

    pub async fn produce(&self, job: i32) -> String {
        let client = reqwest::Client::builder().build();
        let mut req: HashMap<&str, i32> = HashMap::new();
        req.insert("job", job);
        let resp = client
            .unwrap()
            .post(self.url.to_owned())
            .header(CONTENT_TYPE, "application/json")
            .body(serde_json::to_string(&req).unwrap())
            .send()
            .await;
        let r = resp.unwrap().text().await.unwrap();
        let json: PushResponse = serde_json::from_str(r.as_str()).unwrap();
        json.success
    }
}
