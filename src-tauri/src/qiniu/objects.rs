use qiniu_sdk::objects::{apis::credential::Credential, ObjectsManager};
use serde::ser::{Serialize, SerializeStruct, Serializer};

use super::env::{qiniu_env, QiniuEnv};

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
    let QiniuEnv {
        access_key,
        secret_key,
        bucket_name,
    } = qiniu_env();

    let credential = Credential::new(access_key, secret_key);
    let object_manager = ObjectsManager::builder(credential).build();
    let bucket = object_manager.bucket(bucket_name);
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
