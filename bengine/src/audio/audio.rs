/*
* Deals witht he audio of the program
*/

use std::{io::BufReader, fs::File};

use rodio::{OutputStream, Decoder, OutputStreamHandle, Source};

// Struct to play music
pub struct Music{
    stream_handle: OutputStreamHandle,
    song_data: Option<Decoder<BufReader<File>>>,
}

// Methods for the music struct
impl Music{
    // Create a new music object with a song specified
    pub fn new(song_file: &str) -> Self{
        // Create the output stream
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        // Read the encoded song into a BufReader
        let encoded_song_data = BufReader::new(File::open(song_file).unwrap());
        // Decode the encoded song data
        let song_data = Decoder::new(encoded_song_data).unwrap();
        // Wrap into an Option thingy
        let song_data = Some(song_data);
        // Return data
        return Self{
            stream_handle,
            song_data,
        };
    }
    // Run the music object
    pub fn run(mut self){
        // Take the song data out of the music object
        let song_data = self.song_data.take().unwrap();
        // Play the music
        let _result = self.stream_handle.play_raw(song_data.convert_samples());
    }
}
