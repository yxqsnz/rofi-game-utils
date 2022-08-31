use std::{io, process::Command, sync::RwLock};

static OUTPUT: RwLock<String> = RwLock::new(String::new());

const GET_INFO_COMMAND: &str = "radeontop -d - -l 1 -i 0 -";

fn io_custom(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::Other, message)
}

fn io_utf8(data: Vec<u8>) -> io::Result<String> {
    String::from_utf8(data)
        .map_err(|_| io_custom("Failed to convert string to a valid UTF-8 Stream."))
}

// caching radeontop response, so we don't need to run radeontop everytime when we want to get the
// output.
pub fn fetch_info() -> io::Result<()> {
    let command = Command::new("bash")
        .arg("-c")
        .arg(GET_INFO_COMMAND)
        .output()?;

    let output =
        io_utf8(command.stdout).map(|item| item.lines().skip(1).next().unwrap().to_string())?;

    let mut global = OUTPUT
        .write()
        .map_err(|_| io_custom("Failed lock output"))?;
    *global = output;
    Ok(())
}

fn skip_last(string: &str) -> String {
    let mut chars = string.chars();
    chars.next_back();
    chars.collect()
}

pub fn get_vram_usage() -> io::Result<String> {
    let raw_output = OUTPUT
        .read()
        .map_err(|_| io_custom("Failed to lock output"))?;

    println!("Info[radeontop]: Parsing {raw_output}");

    let vram = raw_output
        .split_whitespace()
        .skip(27)
        .next()
        .ok_or_else(|| io_custom("Radeontop sent a invalid response."))?;

    Ok(skip_last(vram))
}

pub fn get_gpu_usage() -> io::Result<String> {
    let raw_output = OUTPUT
        .read()
        .map_err(|_| io_custom("Failed to lock output"))?;

    let usage = raw_output
        .split_whitespace()
        .skip(4)
        .next()
        .ok_or_else(|| io_custom("Radeontop sent a invalid output"))?;

    Ok(skip_last(&usage))
}
