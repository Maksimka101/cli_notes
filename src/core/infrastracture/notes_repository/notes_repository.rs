use kv::{Bucket, Config, Integer, Raw, Store};

use crate::core::domain::note::Note;

pub struct NotesStorage<'a> {
    store: Bucket<'a, Integer, Raw>,
}

impl NotesStorage<'_> {
    pub fn new<'a>(storage_config: Config) -> Result<NotesStorage<'a>, kv::Error> {
        Ok(NotesStorage {
            store: Store::new(storage_config.clone())?.bucket(Some("notes-storage"))?,
        })
    }

    pub fn save(&self, note: &Note) -> Result<(), Error> {
        let bytes = note.to_bytes();
        if let Ok(bytes) = bytes {
            if let Err(_) = self.store.set(&note.id.into(), &Raw::from(bytes)) {
                return Err(Error {});
            }
            return Ok(());
        } else {
            return Err(Error {});
        }
    }

    pub fn read(&self, id: &i32) -> Result<Note, Error> {
        let note_bytes = self.store.get(&id.to_owned().into());
        if let Ok(Some(bytes)) = note_bytes {
            if let Some(note) = Note::from_bytes(&bytes.to_vec()) {
                return Ok(note);
            }
        }
        return Err(Error {});
    }

    pub fn remove(&self, id: &i32) -> Result<Option<Note>, Error> {
        let result = self.store.remove(&id.to_owned().into());
        return match result {
            Ok(Some(bytes)) => Ok(Note::from_bytes(&bytes.to_vec())),
            _ => Err(Error {}),
        };
    }

    pub fn read_all(&self) -> Vec<Result<Note, Error>> {
        return self
            .store
            .iter()
            .map(|item_result| {
                if let Ok(item) = item_result {
                    if let Ok(bytes) = item.value::<Raw>() {
                        if let Some(note) = Note::from_bytes(&bytes.to_vec()) {
                            return Ok(note);
                        }
                    }
                }

                return Err(Error {});
            })
            .collect();
    }
}

pub struct Error {}
