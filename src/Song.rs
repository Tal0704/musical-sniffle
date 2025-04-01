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
}

impl Song {
    pub fn p(&self) -> bool {
        println!("fromSong");
        return true;
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
        let mut lib: Vec<Song> = vec![];
        let mut download: Vec<Song> = vec![];
        let s = Song {status: Status::Downloaded, name: String::from("value")};
        download.push(s);
        let s = Song {status: Status::InLibrary, name: String::from("vlue")};
        lib.push(s);

        let songs = combineSongs(&lib, &download);
        assert!(songs.len() != 0);
    }
}
