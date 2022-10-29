use cli_notes::{
    cli::{command_executor::CommandExecutor, command_parser::Command},
    core::infrastracture::notes_repository::notes_repository::NotesStorage,
};
use kv::Config;

fn main() {
    let config = Config::new("./.cli_notes_storage");

    if let Ok(repository) = NotesStorage::new(config) {
        let command_executor = CommandExecutor { repository };
        if let Ok(command) = Command::parse() {
            let _ = command_executor.execute(command);
        } else {
            println!("Failed to parse the command");
        }
    } else {
        println!("Failed to create notes storage");
    }
}
