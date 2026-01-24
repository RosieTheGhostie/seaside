pub type Tag = [u8; 4];

pub trait Tagged {
    const TAG: Tag;
}
