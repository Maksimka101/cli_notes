use std::{env, io};

use crate::core::domain::note::CreatedNote;

pub enum Command {
    ShowAll,
    Help { due_to_failure: bool },
    Create { created_note: CreatedNote },
    Read { note_id: i32 },
    Remove { note_id: i32 },
}

impl Command {
    pub fn parse() -> Result<Command, CommandParseError> {
        let args = env::args().skip(1).collect::<Vec<String>>();
        return match args.len() {
            1 => match args[0].as_str() {
                "list" => Ok(Command::ShowAll),
                "-h" | "--help" => Ok(Command::Help {
                    due_to_failure: false,
                }),
                "create" => Ok(Command::Create {
                    created_note: Self::read_create_note()?,
                }),
                _ => Ok(Command::Help {
                    due_to_failure: true,
                }),
            },
            2 => {
                let id_result = args[1].parse::<i32>();

                return match id_result {
                    Err(_) => Err(CommandParseError()),
                    Ok(id) => {
                        return match args[0].as_str() {
                            "remove" => Ok(Command::Remove { note_id: id }),
                            "read" => Ok(Command::Read { note_id: id }),
                            _ => Ok(Command::Help {
                                due_to_failure: true,
                            }),
                        };
                    }
                };
            }
            _ => Ok(Command::Help {
                due_to_failure: true,
            }),
        };
    }

    fn read_create_note() -> Result<CreatedNote, CommandParseError> {
        let stdin = io::stdin();
        let mut title = String::new();
        let mut message = String::new();

        println!("Enter the note title then press enter and enter the message");

        if let Err(_) = stdin.read_line(&mut title) {
            return Err(CommandParseError());
        }

        if let Err(_) = stdin.read_line(&mut message) {
            return Err(CommandParseError());
        }

        return Ok(CreatedNote {
            title: title.trim().to_string(),
            message: message.trim().to_string(),
        });
    }
}

pub struct CommandParseError();
