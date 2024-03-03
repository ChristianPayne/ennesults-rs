pub mod file {  
  use serde::{de::DeserializeOwned, Serialize};
  use std::{fs::File, io::{Read, Write}, path::Path};
  
  const BASE_FILE_PATH: &str = "./data";

  pub fn to_json<T: Serialize>(data: T) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(&data)
  }
  
  pub fn create_file(file_name: &str, json: String) -> Result<(), std::io::Error>{
    let mut f = File::create(format!("{}/{}", BASE_FILE_PATH, file_name))?;
    f.write_all(json.as_bytes())
  }
  
  pub fn read_json_file<T, P>(file_path: P) -> Result<T, Box<dyn std::error::Error>>
  where
      T: DeserializeOwned,
      P: AsRef<Path>,
  {
      // Open the file
      let mut file = File::open(file_path)?;
  
      // Read the file contents into a string
      let mut contents = String::new();
      file.read_to_string(&mut contents)?;
  
      // Parse the JSON string using serde_json
      let data: T = serde_json::from_str(&contents)?;
  
      Ok(data)
  }
  
}