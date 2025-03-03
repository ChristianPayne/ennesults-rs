use serde::{de::DeserializeOwned, Serialize};
use std::{
    fs::{read_to_string, remove_file, File},
    io::Write,
};
use tauri::{AppHandle, Manager};

// const BASE_FILE_PATH: &str = "./data";

#[derive(Debug)]
pub enum WriteFileError {
    FailedCreateFile,
    FailedConvertJSON,
    FailedWriteFile,
}

pub fn to_json<T: Serialize>(data: T) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(&data)
}

pub fn write_file<T: Serialize>(
    app_handle: &AppHandle,
    file_name: &str,
    contents: T,
) -> Result<(), WriteFileError> {
    // Get a resource path for where the files will live.
    let resource_path = app_handle
        .path()
        .app_data_dir()
        .expect("Can't resolve app data dir.");
    let full_path = format!(
        "{}/{}",
        resource_path.to_str().expect("Can't convert to str"),
        file_name
    );

    let f_result = File::create(full_path);
    let mut file = match f_result {
        Ok(file) => file,
        Err(_) => return Err(WriteFileError::FailedCreateFile),
    };
    let json = match to_json(contents) {
        Ok(json) => json,
        Err(_) => return Err(WriteFileError::FailedConvertJSON),
    };
    match file.write_all(json.as_bytes()) {
        Ok(_) => Ok(()),
        Err(_) => Err(WriteFileError::FailedWriteFile),
    }
}

pub fn read_json_file<T>(
    app_handle: &AppHandle,
    file_name: &str,
) -> Result<T, Box<dyn std::error::Error>>
where
    T: DeserializeOwned + Default,
{
    // Get a resource path for where the files will live.
    let resource_path = app_handle
        .path()
        .app_data_dir()
        .expect("Can't resolve app data dir.");
    let full_path = format!(
        "{}/{}",
        resource_path.to_str().expect("Can't convert to str"),
        file_name
    );

    let file_contents = read_to_string(full_path)?;

    // Parse the JSON string using serde_json
    let data: T = serde_json::from_str(&file_contents)?;

    Ok(data)
}

pub fn delete_file(
    app_handle: &AppHandle,
    file_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Get a resource path for where the files will live.
    let resource_path = app_handle
        .path()
        .app_data_dir()
        .expect("Can't resolve app data dir.");
    let full_path = format!(
        "{}/{}",
        resource_path.to_str().expect("Can't convert to str"),
        file_name
    );

    remove_file(full_path)?;

    Ok(())
}
