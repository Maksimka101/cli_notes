use cli_notes::{
    cli::{command_executor::CommandExecutor, command_parser::Command},
    core::infrastracture::notes_repository::notes_repository::NotesStorage,
};
use kv::Config;

fn main() {
    let config = Config::new("./.cli_notes_storage");

    if let Ok(repository) = NotesStorage::new(config) {
        let command_executor = CommandExecutor { repository };
        match Command::read_from_args() {
            Ok(command) => {
                let _ = command_executor.execute(command);
            }
            Err(err) => println!("{}", err),
        };
    } else {
        println!("Failed to create notes storage");
    }
}
