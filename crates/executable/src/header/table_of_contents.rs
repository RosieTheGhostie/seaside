#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TableOfContents {
    pub entries: Vec<Entry>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Entry {
    pub tag: [u8; 4],
    pub offset: u32,
    pub size: u32,
}
