#![allow(unused_imports)]
#![allow(non_snake_case)]

use std::mem;
use reqwest::blocking::Client;

#[derive(Clone, PartialEq, Debug)]
pub enum Status {
    InBoth,
    InLibrary,
    Downloaded,
}

#[derive(Clone, PartialEq)]
pub struct Song {
    pub status: Status,
    pub name: String,
    pub artist: String,
    pub album: String,
    pub URL: String,
    pub year: u8,
    pub trackNumber: u8,
    pub imageUrl: String,
}

fn toUri(str: String) -> String {
    return str.replace(" ", "%20");
}

fn fetchUrl(url: String) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    eprintln!("Fetching {url:?}...");

    let client = Client::builder()
        .user_agent("Music Downloaded <0.1>")
        .build()?;

    let res = client.get(url)
        .send()?
        .json::<serde_json::Value>()?;

    Ok(res)
}

impl Song {
    fn download(&mut self, _path: String) {

    }

    fn new() -> Song {
        return Song{status: Status::InBoth,
            name: String::from(""),
            artist: String::from(""),
            album: String::from(""),
            URL: String::from(""),
            imageUrl: String::from(""),
            year: 0,
            trackNumber: 0
        };
    }

    fn getMetadata(&mut self) {
        let url = format!("https://musicbrainz.org/ws/2/release/?query=artist:{}%20AND%20release:{}&fmt=json", toUri(self.artist.clone()), toUri(self.album.clone()));
        let res = fetchUrl(url).expect("Couldn't fetch URL");
        println!("{res}");
    }
}

fn combineSongs(library: &Vec<Song>, downloaded: &Vec<Song>) -> Vec<Song> {
     let mut songs: Vec<Song> = vec![];

     for i in 0..library.len() {
        let mut song = library[i].clone();
        song.status = Status::InLibrary;
        songs.push(song);
     }

     for i in 0..downloaded.len() {
        let mut found = songs.iter_mut().filter(|s| s.name == downloaded[i].name).peekable();
        if found.peek().is_some() {
            found.next().unwrap().status = Status::InBoth;
        }
        else {
            let mut song = downloaded[i].clone();
            song.status = Status::Downloaded;
            songs.push(song);
        }
     }

     return songs;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testCombineSongs() {
        // let mut lib: Vec<Song> = vec![];
        // let mut download: Vec<Song> = vec![];
        // let s = Song {status: Status::Downloaded, name: String::from("value"), URL: String::from("")};
        // download.push(s);
        // let s = Song {status: Status::InLibrary, name: String::from("vlue"), URL: String::from("")};
        // lib.push(s);

        // let songs = combineSongs(&lib, &download);
        // assert!(songs.len() != 0);
    }

    #[test]
    fn testConvertToUri() {
        let mut song = Song::new();
        song.name = String::from("Master of puppets");
        song.artist = String::from("Metallica");
        song.album = String::from("Master of puppets");
        song.getMetadata();
        assert_eq!(song.name.len(), 16)
    }
}
