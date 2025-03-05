use reqwest::blocking::get;
use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]  // 抑制该结构体的蛇形命名警告
pub struct ApiResponse {
    #[serde(rename = "userId")]  // 序列化时字段名为 "userId"
    pub user_id: u32,
    pub id: u32,
    pub title: String,
    pub completed: bool
}

pub fn fetch_data(url: &str) -> Result<ApiResponse, Error> {
    let response = get(url)?.json()?;
    Ok(response)
}
