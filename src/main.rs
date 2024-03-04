use std::io::{self, BufRead, Write};
use std::process::Command;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let handle = stdin.lock();

    let mut command_string = String::new();

    for line in handle.lines() {
        command_string += &line?;
        command_string.push_str("\n");
    }

    let tty = std::fs::OpenOptions::new().read(true).write(true).open("/dev/tty")?;
    let mut tty_reader = io::BufReader::new(&tty);
    let mut tty_writer = io::BufWriter::new(&tty);

    writeln!(tty_writer, "You are about to execute:\n\n{}.\n\nAre you sure? (y/n)", command_string.trim())?;
    tty_writer.flush()?;

    let mut confirmation = String::new();
    tty_reader.read_line(&mut confirmation)?;

    if confirmation.trim().eq_ignore_ascii_case("y") {
        let output = if cfg!(target_os = "windows") {
            // NB. Not tested on Windows
            Command::new("cmd").args(["/C", &command_string]).output()?
        } else {
            Command::new("sh").arg("-c").arg(&command_string).output()?
        };

        io::stdout().write_all(&output.stdout)?;
        io::stderr().write_all(&output.stderr)?;
    } else {
        writeln!(tty_writer, "Command execution cancelled.")?;
    }

    Ok(())
}
