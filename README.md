# Cli notes written in Rust

```
» cli_notes -h  
Usage: cli_notes [command]
  -h --help           show the help message (this message)
  list                list all notes
  create              create a note
  read [note_id]      show note with [note_id]
  remove [note_id]    remove note with [note_id]
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
```
