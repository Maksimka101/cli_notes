use chrono::Local;
use rand::Rng;

use crate::core::{
    domain::note::{CreatedNote, Note},
    infrastracture::notes_repository::notes_repository::NotesStorage,
};

use super::command_parser::Command;

pub struct CommandExecutor<'a> {
    pub repository: NotesStorage<'a>,
}

impl CommandExecutor<'_> {
    pub fn execute(&self, command: Command) -> Result<(), CommandExecutorError> {
        return match command {
            Command::ShowAll => self.show_all(),
            Command::Help { due_to_failure } => Self::show_help(due_to_failure),
            Command::Create { created_note } => self.create(created_note),
            Command::Read { note_id } => self.read(note_id),
            Command::Remove { note_id } => self.remove(note_id),
        };
    }

    fn show_help(show_error_message: bool) -> Result<(), CommandExecutorError> {
        if show_error_message {
            println!("Failed to parse the command");
        }
        println!("Usage: cli_notes [command]");
        println!("  -h --help           show the help message (this message)");
        println!("  list                list all notes");
        println!("  create              create a note");
        println!("  read [note_id]      show note with [note_id]");
        println!("  remove [note_id]    remove note with [note_id]");
        Ok(())
    }

    fn show_all(&self) -> Result<(), CommandExecutorError> {
        for note_option in self.repository.read_all() {
            match note_option {
                Ok(note) => println!("{}", note),
                Err(_) => println!("Failed to read the note"),
            }
            println!("---");
        }

        return Ok(());
    }

    fn remove(&self, note_id: i32) -> Result<(), CommandExecutorError> {
        match self.repository.remove(&note_id) {
            Ok(_) => {
                println!("The note successfully removed");
                return Ok(());
            }
            Err(_) => {
                println!("Failed to remove the note");
                return Err(CommandExecutorError());
            }
        }
    }

    fn read(&self, note_id: i32) -> Result<(), CommandExecutorError> {
        match self.repository.read(&note_id) {
            Err(_) => {
                println!("Failed to read the note");
                return Err(CommandExecutorError());
            }
            Ok(note) => {
                println!("{}", note);
                return Ok(());
            }
        }
    }

    fn create(&self, created_note: CreatedNote) -> Result<(), CommandExecutorError> {
        let mut rng = rand::thread_rng();
        let note = Note {
            id: rng.gen(),
            created_at: Local::now().to_string(),
            updated_at: Local::now().to_string(),
            title: created_note.title,
            message: created_note.message,
        };

        return match self.repository.save(&note) {
            Err(_) => {
                println!("Failed to save the note");
                Err(CommandExecutorError())
            }
            Ok(_) => {
                println!("Note successfully created");
                Ok(())
            }
        };
    }
}

pub struct CommandExecutorError();
