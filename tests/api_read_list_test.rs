mod src;

#[cfg(test)]
mod tests {
    use actix_web::{
        test::{self, TestRequest},
        web, App,
    };

    use redis_client::{handlers::redis::read::read_list, models::response::Response};

    use crate::src::{
        common::{load_test_params, TestSetup},
        data_structures::read::ReadKey,
    };

    #[actix_rt::test]
    async fn api_read_list_test() {
        let (host_redis, port_redis) = load_test_params();
        let test_setup = TestSetup::new(host_redis, port_redis);
        test_setup.drop();
        test_setup.setup_test_list_data("read_key", vec!["three", "two", "one"]);

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(test_setup.init_pool()))
                .service(read_list),
        )
        .await;

        let test_cases = [
            (
                "Проверка чтения из начала очереди.".to_string(),
                ReadKey::default().key("read_key").read_mod("FIRST"),
                Response::default()
                    .status("OK")
                    .message("Data read successfully.")
                    .data("one"),
            ),
            (
                "Проверка чтения из конца очереди.".to_string(),
                ReadKey::default().key("read_key").read_mod("LAST"),
                Response::default()
                    .status("OK")
                    .message("Data read successfully.")
                    .data("three"),
            ),
            (
                "Проверка чтения из пустой базы.".to_string(),
                ReadKey::default().key("error_key").read_mod("LAST"),
                Response::default()
                    .status("KO")
                    .message("The key 'error_key' does not exist or is empty!")
                    .data(""),
            ),
            (
                "Проверка некорректного add_mode.".to_string(),
                ReadKey::default()
                    .key("read_key")
                    .read_mod("ERROR_READ_MODE"),
                Response::default()
                    .status("KO")
                    .message("Incorrect operation (ERROR_READ_MODE), expected (FIRST, LAST)!")
                    .data(""),
            ),
        ];

        for (_, read_data, response) in test_cases {
            let req = TestRequest::get()
                .uri(&format!(
                    "/readList?key={}&read_mode={}",
                    read_data.get_key(),
                    read_data.get_read_mod()
                ))
                .to_request();

            let resp = test::call_service(&app, req).await;
            let response_body = test::read_body(resp).await;
            let result: Response = serde_json::from_slice(&response_body).unwrap();

            // Проверяем ответ от сервера
            assert_eq!(result, response);
        }
        test_setup.drop();
    }
}