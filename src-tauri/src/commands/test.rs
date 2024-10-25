use super::{Command, UserLevel};

#[derive(Debug)]
pub struct Test;

impl Command for Test {
    fn get_name(&self) -> String {
        "test".to_string()
    }
    fn get_aliases(&self) -> Option<Vec<String>> {
        None
    }
    fn get_required_user_level(&self) -> UserLevel {
        UserLevel::Moderator
    }
    fn run(&self) -> Option<String> {
        None
    }
}

#[derive(Debug)]
pub struct SecondTest;

impl Command for SecondTest {
    fn get_name(&self) -> String {
        "secondtest".to_string()
    }
    fn get_aliases(&self) -> Option<Vec<String>> {
        Some(vec!["second".to_string()])
    }
    fn get_required_user_level(&self) -> UserLevel {
        UserLevel::Viewer
    }
    fn run(&self) -> Option<String> {
        Some("Here is what I have to say".to_string())
    }
}
