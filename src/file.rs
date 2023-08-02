use std::{
    ffi::{OsStr, OsString},
    path::Path,
};

use serde::{Deserialize, Serialize, Serializer};
use tokio::fs;

use crate::{requests::PddGoodsFilespaceImageUpload, Error};

#[derive(Deserialize, Debug)]
pub struct PddFile {
    file_data: Vec<u8>,
    pub file_name: OsString,
}

impl PddFile {
    pub fn new(file_data: Vec<u8>, file_name: OsString) -> Self {
        PddFile {
            file_data,
            file_name,
        }
    }

    pub async fn from_file(path: &str) -> Result<Self, Error> {
        let path = Path::new(path);
        let file_data = fs::read(path).await?;
        let file_name = match path.file_name() {
            Some(p) => p,
            None => OsStr::new("name.jpg"),
        };
        Ok(PddFile {
            file_data,
            file_name: file_name.to_os_string(),
        })
    }
}

impl Serialize for PddFile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("")
    }
}

pub trait FileUploadRequest {
    fn get_file(self) -> Option<(String, Vec<u8>)>;
}

impl FileUploadRequest for PddGoodsFilespaceImageUpload {
    fn get_file(self) -> Option<(String, Vec<u8>)> {
        //TODO: 文件名unwrap
        self.file
            .map(|file| (file.file_name.into_string().unwrap(), file.file_data))
    }
}
