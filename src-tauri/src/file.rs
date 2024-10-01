use serde::{de::DeserializeOwned, Serialize};
use std::{
    fs::{self, File, read_to_string},
    io::{Read, Write},
    path::Path,
};
use tauri::{AppHandle, Manager};

// const BASE_FILE_PATH: &str = "./data";

pub enum WriteFileError {
    FailedCreateFile,
    FailedConvertJSON,
    FailedWriteFile
}

pub fn to_json<T: Serialize>(data: T) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(&data)
}

pub fn write_file<T: Serialize>(app_handle: &AppHandle, file_name: &str, contents: T) -> Result<(), WriteFileError> {
    // Get a resource path for where the files will live.
    let resource_path = app_handle.path().app_data_dir().expect("Can't resolve app data dir.");
    let full_path = format!("{}/{}",resource_path.to_str().expect("Can't convert to str"), file_name);

    let f_result = File::create(full_path);
    let mut file = match f_result {
        Ok(file) => {
            file
        },
        Err(_) => {
            return Err(WriteFileError::FailedCreateFile)
        }
    };
    let json = match to_json(contents) {
        Ok(json) => json,
        Err(_) => return Err(WriteFileError::FailedConvertJSON)
    };
    match file.write_all(json.as_bytes()) {
        Ok(_) => Ok(()),
        Err(_) => Err(WriteFileError::FailedWriteFile)
    }
}

pub fn read_json_file<T>(app_handle: &AppHandle, file_name: &str) -> Result<T, Box<dyn std::error::Error>> where T: DeserializeOwned
{
    // Get a resource path for where the files will live.
    let resource_path = app_handle.path().app_data_dir().expect("Can't resolve app data dir.");
    let full_path = format!("{}/{}", resource_path.to_str().expect("Can't convert to str"), file_name);

    dbg!(&full_path);

    let file_contents = read_to_string(full_path)?;
    
    // Parse the JSON string using serde_json
    let data: T = serde_json::from_str(&file_contents)?;

    Ok(data)
}

// R&D

// Get a resource path for where the files will live.
// let resource_path = app.path_resolver().app_data_dir().expect("Can't resolve app data dir.");
// let full_path = format!("{}/test.json", resource_path.to_str().expect("Can't convert to str"));

// println!("Files for the app will be saved here: {}", &full_path);

// if Path::new(&full_path).exists() == false {
//   fs::write(&full_path, "{}").expect("Failed to write file.")
// }

// let message: String = fs::read_to_string(&full_path).expect("Failed to read string from file path.");

            // dbg!(message);

            // let test_data: bool = false;

            // file::create_file(
            //     app.handle(),
            //     "test_json_file",
            //     file::to_json(test_data).expect("Failed to convert to json."),
            // )
            // .expect("failed to create file");

            // app.handle().path_resolver().app_data_dir()

            // let contents = read_string("$DESKTOP/test/test.json");

            // dbg!(contents);