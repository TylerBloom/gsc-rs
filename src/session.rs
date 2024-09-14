use std::{path::Path, process::Command};

use crate::script::Script;


pub struct Session {
    script: Script
}

impl Session {
    pub fn new(script: impl AsRef<Path>, shell: Command) -> Self {
        todo!()
    }

    pub fn run(&mut self) -> Result<(), ()> {
        while !self.script.at_eof() {
            if self.handle_script_line() {
                return Ok(())
            }
            while self.script
            todo!()
        }
        todo!()
    }

    pub fn cleanup(self) {
        todo!()
    }
}

pub struct TerminalSession {
}
