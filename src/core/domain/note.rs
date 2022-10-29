use std::fmt::Display;

use bytecheck::CheckBytes;
use rkyv::{ser::serializers::AllocSerializer, Archive, Deserialize, Fallible, Serialize};

#[derive(Debug, Archive, Serialize, Deserialize)]
#[archive_attr(derive(CheckBytes))]
pub struct Note {
    pub id: u32,
    pub edited_at: String,
    pub title: String,
    pub message: String,
}

impl Note {
    pub fn from_bytes(bytes: &Vec<u8>) -> Option<Note> {
        let note_deserializer = rkyv::check_archived_root::<Note>(bytes);
        if let Ok(deserializer) = note_deserializer {
            let deserialized_note: Note = deserializer.deserialize(&mut rkyv::Infallible).unwrap();
            return Some(deserialized_note);
        }

        return None;
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, <AllocSerializer<256> as Fallible>::Error> {
        let bytes = rkyv::to_bytes::<_, 256>(self)?;
        let bytes_vec = bytes.into_vec();
        return Ok(bytes_vec);
    }
}

impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Id: {}\nCreated at: {}\nTitle: {}\nMessage: {}",
            self.id, self.edited_at, self.title, self.message,
        )
    }
}

pub struct CreatedNote {
    pub title: String,
    pub message: String,
}
