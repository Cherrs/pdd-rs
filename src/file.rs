use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PddFile {
    file_data: Vec<u8>,
}

impl PddFile {
    pub fn new(file_data: Vec<u8>) -> Self {
        PddFile { file_data }
    }
}
