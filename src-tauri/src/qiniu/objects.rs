use qiniu_sdk::objects::{apis::credential::Credential, ObjectsManager};
use serde::ser::{Serialize, SerializeStruct, Serializer};

use super::env::QiniuEnv;

pub struct File {
    name: String,
}

impl Serialize for File {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("File", 1)?;
        state.serialize_field("name", &self.name)?;
        state.end()
    }
}

#[tauri::command]
pub fn hello() -> Vec<File> {
    let mut qiniuenv = QiniuEnv::new("".to_string(), "".to_string(), "".to_string());

    match QiniuEnv::qiniu_env() {
        Err(err) => {
            print!("err: {}", err);
            return vec![File {
                name: err.to_string(),
            }];
        }
        Ok(env) => {
            qiniuenv.access_key = env.access_key;
            qiniuenv.bucket_name = env.bucket_name;
            qiniuenv.secret_key = env.secret_key;
        }
    }

    let credential = Credential::new(qiniuenv.access_key, qiniuenv.secret_key);
    let object_manager = ObjectsManager::builder(credential).build();
    let bucket = object_manager.bucket(qiniuenv.bucket_name);
    let mut iter = bucket.list().iter();
    let mut files = Vec::new();
    while let Some(object) = iter.next() {
        let object = object.unwrap();
        println!("fsize: {:?}", object);
        files.push(File {
            name: object.get_key_as_str().to_string(),
        });
    }
    files
}
