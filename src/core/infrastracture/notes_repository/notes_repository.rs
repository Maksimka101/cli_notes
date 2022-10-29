use kv::{Bucket, Config, Integer, Raw, Store};

use crate::{core::domain::note::Note, DynError, Failure};

pub struct NotesStorage<'a> {
    store: Bucket<'a, Integer, Raw>,
}

impl NotesStorage<'_> {
    pub fn new<'a>(storage_config: Config) -> Result<NotesStorage<'a>, kv::Error> {
        Ok(NotesStorage {
            store: Store::new(storage_config.clone())?.bucket(Some("notes-storage"))?,
        })
    }

    pub fn save(&self, note: &Note) -> Result<(), DynError> {
        let bytes = note.to_bytes()?;
        self.store.set(&note.id.into(), &Raw::from(bytes))?;
        Ok(())
    }

    pub fn read(&self, id: &i32) -> Result<Note, DynError> {
        if let Ok(Some(bytes)) = self.store.get(&id.to_owned().into()) {
            if let Some(note) = Note::from_bytes(&bytes.to_vec()) {
                return Ok(note);
            }
        }
        return Err(Failure.into());
    }

    pub fn remove(&self, id: &i32) -> Result<Option<Note>, DynError> {
        let result = self.store.remove(&id.to_owned().into());
        return match result {
            Ok(Some(bytes)) => Ok(Note::from_bytes(&bytes.to_vec())),
            _ => Err(Failure.into()),
        };
    }

    pub fn read_all(&self) -> Vec<Result<Note, DynError>> {
        return self
            .store
            .iter()
            .map(|item_result| {
                let bytes = item_result?.value::<Raw>()?;
                let note = Note::from_bytes(&bytes.to_vec());
                return note.ok_or_else(|| Failure.into());
            })
            .collect();
    }
}
