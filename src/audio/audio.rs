use std::{io::BufReader, fs::File};

use rodio::{OutputStream, Decoder, OutputStreamHandle, Source};

pub struct Music{
    stream_handle: OutputStreamHandle,
    song_data: Option<Decoder<BufReader<File>>>,
}

impl Music{
    pub fn new(song_file: &str) -> Self{
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let encoded_song_data = BufReader::new(File::open(song_file).unwrap());
        let song_data = Decoder::new(encoded_song_data).unwrap();
        let song_data = Some(song_data);
        return Self{
            stream_handle,
            song_data,
        };
    }
    pub fn run(mut self){
        let song_data = self.song_data.take().unwrap();
        let _result = self.stream_handle.play_raw(song_data.convert_samples());
    }
}
