pub mod hello_world;

// Command components
#[derive(Clone)]
pub struct CommandName (String);
pub enum AccessLevel {
    User,
    Moderator
}

trait Command {
    fn get_name(self) -> CommandName;
    fn get_aliases(self) -> Vec<String>;
}
