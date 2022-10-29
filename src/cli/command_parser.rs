use std::io;

use structopt::{clap, StructOpt};

use crate::core::domain::note::CreatedNote;

#[derive(StructOpt)]
enum CliCommand {
    /// List all notes.
    #[structopt(name = "list")]
    ShowAll,
    /// Create a new note.
    Create,
    /// Read the note by the given id.
    Read { note_id: i32 },
    /// Remove the note by the given id.
    Remove { note_id: i32 },
}

pub enum Command {
    ShowAll,
    Create { created_note: CreatedNote },
    Read { note_id: i32 },
    Remove { note_id: i32 },
}

impl Command {
    pub fn read_from_args() -> Result<Command, clap::Error> {
        match CliCommand::from_args_safe()? {
            CliCommand::ShowAll => Ok(Command::ShowAll),
            CliCommand::Read { note_id } => Ok(Command::Read { note_id }),
            CliCommand::Remove { note_id } => Ok(Command::Remove { note_id }),
            CliCommand::Create => Ok(Command::Create {
                created_note: Self::read_create_note()?,
            }),
        }
    }

    fn read_create_note() -> Result<CreatedNote, clap::Error> {
        let stdin = io::stdin();
        let mut title = String::new();
        let mut message = String::new();

        println!("Enter the note title then press enter and enter the message");

        if let Err(_) = stdin.read_line(&mut title) {
            return Err(clap::Error {
                message: "Failed to read the title parameter".to_string(),
                kind: clap::ErrorKind::Format,
                info: None,
            });
        }

        if let Err(_) = stdin.read_line(&mut message) {
            return Err(clap::Error {
                message: "Failed to read the message parameter".to_string(),
                kind: clap::ErrorKind::Format,
                info: None,
            });
        }

        return Ok(CreatedNote {
            title: title.trim().to_string(),
            message: message.trim().to_string(),
        });
    }
}

pub struct CommandParseError();
