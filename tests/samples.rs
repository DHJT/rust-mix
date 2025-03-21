use mockall::automock;

#[automock]
pub trait MyTrait {
    fn do_something(&self) -> String;
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;

    #[test]
    fn test_something() {
        let mut mock_my_trait = MockMyTrait::new();
        mock_my_trait.expect_do_something().returning(|| "mocked".to_string());
        let result = mock_my_trait.do_something();
        assert_eq!(result, "mocked");
    }

    use mockito::{Server};
    use reqwest; // 假设你使用 reqwest 进行 HTTP 请求

    #[test]
    fn test_http_request() {
        // 启动一个 mock server
        let mut server = Server::new();
        let _m = server.mock("GET", "/some/endpoint")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("{\"name\": \"mock\"}")
            .create();
        let client = reqwest::blocking::Client::builder()
            .build().unwrap();
        // 发送请求到 mock server
        let response = client.get(server.url() + "/some/endpoint").send().unwrap();
        eprintln!("Response: {:?}", &response);
        assert_eq!((&response).status(), 200); // 检查响应状态码
    }

}
