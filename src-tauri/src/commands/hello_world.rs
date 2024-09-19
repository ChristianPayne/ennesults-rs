use crate::commands::*;

pub struct HelloWorld {
    name: CommandName,
    aliases: Vec<&'static str>,
}

impl Command for HelloWorld {
    fn get_name(self) -> CommandName {
        self.name.clone()
    }

    fn get_aliases(self) -> Vec<&'static str> {
        self.aliases.clone()
    }
}

impl Default for HelloWorld {
    fn default() -> Self {
        HelloWorld {
            name: CommandName("hello_world".to_owned()),
            aliases: vec!["hello"],
        }
    }
}

impl HelloWorld {
    fn say_hello_to(name: String) -> String {
        format!("Hello {}", name)
    }
}
