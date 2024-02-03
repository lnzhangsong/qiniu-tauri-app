use std::env;

use dotenvy::dotenv;

pub struct QiniuEnv {
    pub access_key: String,
    pub secret_key: String,
    pub bucket_name: String,
}

pub fn qiniu_env() -> QiniuEnv {
    dotenv().expect("src-tauri/.env not found");
    let access_key = env::var("access_key").expect("access_key not found");
    let secret_key = env::var("secret_key").expect("secret_key not found");
    let bucket_name = env::var("bucket_name").expect("bucket_name not found");
    QiniuEnv {
        access_key,
        secret_key,
        bucket_name,
    }
}
