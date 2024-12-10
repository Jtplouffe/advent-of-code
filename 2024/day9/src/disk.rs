use crate::block::empty::EmptyBlock;
use crate::block::file::File;
use crate::block::Block;

#[derive(Clone)]
pub struct Disk {
    blocks: Vec<Block>,
}

impl Disk {
    pub fn from_map(disk_map: &str) -> Self {
        let mut file_id = 0;
        let mut blocks = Vec::new();

        let mut is_file = true;
        for c in disk_map.chars() {
            let width = c.to_digit(10).expect("Not a digit") as usize;

            match is_file {
                true => {
                    blocks.push(File::new(file_id, width).into());
                    file_id += 1;
                }
                false => blocks.push(EmptyBlock::new(width).into()),
            }

            is_file = !is_file;
        }

        Self { blocks }
    }

    pub fn compact_partial_files(&mut self) {
        let mut head_index = 0;
        let mut tail_index = self.blocks.len() - 1;

        while head_index != tail_index {
            if self.blocks[head_index].is_file() {
                head_index += 1;
                continue;
            }

            if self.blocks[tail_index].is_empty() {
                tail_index -= 1;
                continue;
            }

            let head_empty_width = self.blocks[head_index].empty().width;

            let tail_block = self.blocks.remove(tail_index);
            let tail_file = tail_block.file();

            let (split, remaining) = tail_file.split(head_empty_width.min(tail_file.width));

            let split_width = split.width;

            if split.width == head_empty_width {
                self.blocks[head_index] = split.into();
            } else {
                self.blocks.insert(head_index, split.into());

                // Since we inserted, we must move back the tail index by 1
                tail_index += 1;

                self.blocks[head_index + 1]
                    .empty_mut()
                    .shrink_by(split_width);
            }

            if let Some(remaining) = remaining {
                self.blocks.insert(tail_index, remaining.into());
            } else {
                tail_index -= 1;
            }

            head_index += 1
        }
    }

    pub fn compact_whole_files(&mut self) {
        let mut first_empty_block_index = None;
        let mut head_index = 0;
        let mut tail_index = self.blocks.len() - 1;

        while tail_index > 0 {
            if head_index >= tail_index {
                tail_index -= 1;
                head_index = first_empty_block_index.unwrap_or_default();
            }

            if self.blocks[head_index].is_file() {
                head_index += 1;
                continue;
            }

            if self.blocks[tail_index].is_empty() {
                tail_index -= 1;
                continue;
            }

            let head_empty_width = self.blocks[head_index].empty().width;
            let tail_file_width = self.blocks[tail_index].file().width;

            if tail_file_width > head_empty_width {
                if first_empty_block_index.is_none() {
                    first_empty_block_index = Some(head_index);
                }

                head_index += 1;
                continue;
            }

            let tail_block = self.blocks.remove(tail_index);

            if tail_file_width == head_empty_width {
                let head_block = self.blocks.remove(head_index);

                self.blocks.insert(head_index, tail_block);
                self.blocks.insert(tail_index, head_block);

                tail_index -= 1;
            } else {
                self.blocks.insert(head_index, tail_block);

                self.blocks[head_index + 1]
                    .empty_mut()
                    .shrink_by(tail_file_width);
                self.blocks.insert(tail_index, EmptyBlock::new(tail_file_width).into());
            }

            head_index = first_empty_block_index.unwrap_or_default();
        }
    }

    pub fn checksum(&self) -> usize {
        let mut checksum = 0;

        let mut position = 0;
        for block in self.blocks.iter() {
            match block {
                Block::File(file) => {
                    for _ in 0..file.width {
                        checksum += position * file.id;
                        position += 1;
                    }
                }
                Block::Empty(empty_block) => {
                    position += empty_block.width;
                }
            }
        }

        checksum
    }
}
