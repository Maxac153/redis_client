mod common;

#[cfg(test)]
mod tests {
    use actix_web::{
        http::StatusCode,
        test::{self, TestRequest},
        web, App,
    };

    use crate::common::common::{load_test_params, TestSetup};
    use redis_client::handlers::redis::download_dump_key::download_dump_all_keys;

    #[actix_rt::test]
    async fn api_download_dump_all_keys_test() {
        let (host_redis, port_redis) = load_test_params();
        let test_setup = TestSetup::new(host_redis, port_redis);
        let keys = vec!["download_dump_all_keys_one", "download_dump_all_keys_two"];
        test_setup.drop();

        for key in keys {
            test_setup.setup_test_list_data(key, vec!["test_data"]);
        }

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(test_setup.init_pool()))
                .service(download_dump_all_keys),
        )
        .await;

        let test_cases = vec![("Проверка создания всего дампа Redis.", StatusCode::OK)];

        for (_, expected_status) in test_cases {
            let req = TestRequest::get().uri("/downloadDumpAllKeys").to_request();
            let result = test::call_service(&app, req).await;
            assert_eq!(result.status(), expected_status);
        }
        test_setup.drop();
    }
}
