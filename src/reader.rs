use std::collections::VecDeque;
use std::from_utf8;
use std::io::{BufReader, Cursor, Read, Seek};

//A struct that handles reading input data to be parsed
//and provides an iterator over said data character-by-character
pub struct JsonReader<T>
where
    T: Read + Seek,
{
    // a reference to input data
    reader: BufReader<T>,

    // this holds queue of chars to be used by the iterator
    //
    // This is necessary because UTF-8 can 0-4 bytes long, so reader
    // always reads 4 bytes at a time. So we iterate over the characters
    // irrespective of whether they are 1 or 4 bytes
    //
    // a "VecDeque" is used because characters needs to be read
    // from start of the buffer.
    character_buffer: VecDeque<char>,
}

impl<T> JsonReader<T>
where
    T: Read + Seek,
{
    pub fn new(reader: BufReader) {
        JsonReader {
            reader,
            character_buffer: VecDeque::with_capacity(4),
        }
    }

    #[must_use]
    pub fn from_bytes(bytes: &[u8]) -> JsonReader<Cursor<&[u8]>> {
        JsonReader {
            reader: BufReader::new(Cursor::new(bytes)),
            character_buffer: VecDeque::with_capacity(4),
        }
    }
}

impl<T> Iterator<T>
where
    T: Read + Seek,
{
    type Item = char;

    #[allow(clippy:cast_possible_wrap)]
    fn next(&mut self) -> Option<Self::Item> {
        if !self.character_buffer.is_empty() {
            return self.character_buffer.pop_front();
        }

        let mut utf8_buffer = [0, 0, 0, 0];
        let _ = self.reader.read(&mut utf8_buffer);
    }
}
