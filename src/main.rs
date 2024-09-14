#![allow(dead_code, unused)] // Test comments

use std::fs::File;
use std::io::{stdin, stdout, BufRead, BufReader, Read, Write};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};

use ptyprocess::PtyProcess;

mod char_tree;
mod script;
mod session;

use session::Session;

// A simple prototype
fn main() {
    // spawn a cat process
    let mut process = PtyProcess::spawn(Command::new("bash")).expect("failed to spawn a process");

    // create a communication stream
    let mut stream = process.get_raw_handle().expect("failed to create a stream");
    let mut stream = Arc::new(stream);

    std::thread::spawn({
        let stream = stream.clone();
        move || output(stream)
    });

    let mut buffer_in = String::new();
    // let mut buffer_out = Vec::with_capacity(1_000_000);
    let mut stdin = stdin();
    loop {
        print!("Enter command $ ");
        stdout().flush().unwrap();
        buffer_in.clear();
        stdin.read_line(&mut buffer_in).unwrap();
        write!(stream, "{buffer_in}").unwrap();
    }
    // stop the process
    assert!(process.exit(true).expect("failed to stop the process"))
}

fn output(stream: Arc<File>) {
    let mut buffer_out = Vec::new();
    let mut reader = BufReader::new(stream);
    let mut stdout = stdout();
    loop {
        buffer_out.clear();
        reader.read_until(b'\n', &mut buffer_out).unwrap();
        stdout.write_all(&buffer_out);
    }
}

/* Direct port
fn main() {
    let shell = make_interactive_shell();
    let session = Session::new("cat", shell);
    let digest = session.run();
    if let Err(()) = &digest {
        eprintln!("Session failed with error");
        session.cleanup();
    }
    digest
}

fn make_interactive_shell() -> Command {
    todo!()
}
*/

/* Example from ptyprocess
fn main() {
    use ptyprocess::PtyProcess;
    use std::io::{BufRead, BufReader, Write};
    use std::process::Command;

    // spawn a cat process
    let mut process = PtyProcess::spawn(Command::new("bash")).expect("failed to spawn a process");

    // create a communication stream
    let mut stream = process.get_raw_handle().expect("failed to create a stream");

    // send a message to process
    writeln!(stream, "touch hello_from_worker\r").expect("failed to write to a stream");

    // read a line from the stream
    let mut reader = BufReader::new(stream);
    let mut buf = String::new();
    reader
        .read_line(&mut buf)
        .expect("failed to read a process output");

    println!("line={}", buf);

    // stop the process
    assert!(process.exit(true).expect("failed to stop the process"))
}
*/
