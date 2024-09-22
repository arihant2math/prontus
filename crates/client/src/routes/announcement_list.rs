pub use crate::Announcement;

struct Cursors {
    pub prev: Option<String>,
    pub next: Option<String>,
}

struct Root {
    pub ok: bool,
    pub announcements: Vec<Announcement>,
    pub pagesize: u64,
    pub hasmore: bool,
    pub cursors: Cursors,
}
