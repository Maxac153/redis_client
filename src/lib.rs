pub mod handlers {
    pub mod pages {
        pub mod index;
    }
    pub mod redis {
        pub mod add;
        pub mod change_ttl;
        pub mod download_dump_key;
        pub mod read;
        pub mod rename_key;
        pub mod reset;
        pub mod status;
        pub mod upload_dump_key;
    }
}

pub mod models {
    pub mod response;
    pub mod status;
    pub mod status_key;
    pub mod type_key;
}

pub mod routes {
    pub mod init_routes_pages;
    pub mod init_routes_redis;
}

pub mod config;
