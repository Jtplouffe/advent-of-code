use empty::EmptyBlock;
use file::File;

pub mod empty;
pub mod file;

#[derive(Clone)]
pub enum Block {
    File(File),
    Empty(EmptyBlock),
}

impl Block {
    pub fn is_file(&self) -> bool {
        matches!(self, Self::File(_))
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty(_))
    }

    pub fn file(&self) -> &File {
        match self {
            Self::File(file) => file,
            _ => panic!("Block is not a file"),
        }
    }

    pub fn empty(&self) -> &EmptyBlock {
        match self {
            Self::Empty(empty_block) => empty_block,
            _ => panic!("Block is not an empty block"),
        }
    }

    pub fn empty_mut(&mut self) -> &mut EmptyBlock {
        match self {
            Self::Empty(empty_block) => empty_block,
            _ => panic!("Block is not an empty block"),
        }
    }
}

impl From<File> for Block {
    fn from(value: File) -> Self {
        Self::File(value)
    }
}

impl From<EmptyBlock> for Block {
    fn from(value: EmptyBlock) -> Self {
        Self::Empty(value)
    }
}
