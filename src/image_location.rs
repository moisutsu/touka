use std::path::PathBuf;

pub enum ImageLocation {
    File(PathBuf),
    Clipboard,
}

impl From<Option<PathBuf>> for ImageLocation {
    fn from(path: Option<PathBuf>) -> Self {
        path.map_or(ImageLocation::Clipboard, ImageLocation::File)
    }
}
