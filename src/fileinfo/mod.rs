#[derive(Clone, Debug)]
pub struct FileInfo<'life> {
    pub data: &'life [u8],
    pub name: String,
    pub dir: String,
}
