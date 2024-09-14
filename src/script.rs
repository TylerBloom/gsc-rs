use std::path::{Path, PathBuf};

use crate::char_tree::CharTree;

pub struct Script {
    filenames: Vec<PathBuf>,
    lines: Vec<String>,
    // add_lines(filename)
    line: usize,
    col: usize,
    multi_char_keys: CharTree, // = CharTree()
}

impl Script {
    pub fn at_eol(&self) -> bool {
        self.line >= self.lines.len() || self.col >= self.lines[self.line].len()
    }

    pub fn at_eof(&self) -> bool {
        self.line >= self.lines.len()
            || (self.line >= self.lines.len() - 1 && self.col >= self.lines[self.line].len())
    }

    pub fn add_lines(&mut self, lines: impl AsRef<Path>) {
        let path = lines.as_ref().to_path_buf();
        self.lines.extend(
            std::fs::read_to_string(&path)
                .unwrap()
                .lines()
                .map(ToOwned::to_owned),
        );
        self.filenames.push(path);
    }

    pub fn reload(&mut self) {
        self.lines.clear();
        self.filenames
            .iter()
            .map(|p| std::fs::read_to_string(p).unwrap())
            .for_each(|data| self.lines.extend(data.lines().map(ToOwned::to_owned)))
    }
}
