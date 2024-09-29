use serde::{de::DeserializeOwned, Serialize};
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};
use tauri::{AppHandle, Manager};

// const BASE_FILE_PATH: &str = "./data";

pub fn to_json<T: Serialize>(data: T) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(&data)
}

pub fn create_file(app: &AppHandle, file_name: &str, json: String) -> Result<(), std::io::Error> {
    // LEFT OFF TRYING TO GET THE APP REFERENCE INTO THIS FUNCTION.

    // Get a resource path for where the files will live.
    let resource_path = app
        .path()
        .app_data_dir()
        .expect("Can't resolve app data dir.");
    let full_path = format!(
        "{}/{}",
        resource_path.to_str().expect("Can't convert to str"),
        file_name
    );

    let mut f = File::create(full_path)?;
    f.write_all(json.as_bytes())
}

pub fn read_json_file<T, P>(app: AppHandle, file_path: P) -> Result<T, Box<dyn std::error::Error>>
where
    T: DeserializeOwned,
    P: AsRef<Path>,
{
    // LEFT OFF TRYING TO GET THE APP REFERENCE INTO THIS FUNCTION.
    // Get a resource path for where the files will live.
    // let resource_path = app.path_resolver().app_data_dir().expect("Can't resolve app data dir.");
    // let full_path = format!("{}/{}", resource_path.to_str().expect("Can't convert to str"), file_name);

    // Open the file
    let mut file = File::open(file_path)?;

    // Read the file contents into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Parse the JSON string using serde_json
    let data: T = serde_json::from_str(&contents)?;

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