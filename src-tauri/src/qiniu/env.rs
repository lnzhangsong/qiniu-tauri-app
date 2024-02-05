use dotenvy::Error;
use dotenvy_macro;

pub struct QiniuEnv {
    pub access_key: String,
    pub secret_key: String,
    pub bucket_name: String,
}

impl QiniuEnv {
    pub fn new(access_key: String, secret_key: String, bucket_name: String) -> QiniuEnv {
        QiniuEnv {
            access_key,
            secret_key,
            bucket_name,
        }
    }

    pub fn qiniu_env() -> Result<QiniuEnv, Error> {
        let access_key = dotenvy_macro::dotenv!("access_key").to_string();
        let secret_key = dotenvy_macro::dotenv!("secret_key").to_string();
        let bucket_name = dotenvy_macro::dotenv!("bucket_name").to_string();
        return Ok(QiniuEnv {
            access_key,
            secret_key,
            bucket_name,
        });
    }
}
