pub enum Status {
    InBoth,
    OnlyInLibrary,
    OnlyDownloaded,
}

pub struct Song {
    pub status: Status,
}

impl Song {
    pub fn p(&self) -> bool {
        println!("fromSong");
        return true;
    }
}
