# Cli notes written in Rust

Usage example:
```
» cli_notes create
Enter the note title then press enter and enter the message
Note showcase
This is an example of the note creation process
Note successfully created

» cli_notes list  
Id: 1757673530
Created at: 2022-10-29 22:36:22.573482 +04:00
Title: Note showcase
Message: This is an example of the note creation process
---
Id: -767085535
Created at: 2022-10-29 17:57:01.226513 +04:00
Title: Title 3
Message: Message number 3
---
» cli_notes remove -767085535           
The note successfully removed

» cli_notes read 1757673530
Id: 1757673530
Created at: 2022-10-29 22:36:22.573482 +04:00
Title: Note showcase
Message: This is an example of the note creation process

» ./cli_notes -h         
cli_notes 0.1.0

USAGE:
    cli_notes <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    create    Create a new note
    help      Prints this message or the help of the given subcommand(s)
    list      List all notes
    read      Read the note by the given id
    remove    Remove the note by the given id
```
