#[derive(Clone)]
pub struct FileInfo<'life> {
    pub data: &'life [u8],
    pub name: String,
}
