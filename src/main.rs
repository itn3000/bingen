use std::io::Write;
use std::io::Read;
use thiserror::Error;

enum AppError {
    
}

enum StdoutOrFile {
    Stdout(std::io::Stdout),
    File(std::fs::File)
}

impl Write for StdoutOrFile {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            Self::Stdout(f) => f.write(buf),
            Self::File(f) => f.write(buf)
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            Self::Stdout(f) => f.flush(),
            Self::File(f) => f.flush()
        }
    }
}

enum InputDataType {
    Hex(String),
    Base64(String),
    File(String)
}

impl InputDataType {
    fn get_bytes(&self) -> Result<Vec<u8>, impl std::error::Error>  {
        match self {
            Self::Hex(s) => Ok(hex::decode(s)?),
            Self::Base64(s) => Ok(base64::decode(s)?),
            Self::File(s) => {
                let f = std::fs::File::open(s)?;
                let fmeta = f.metadata()?;
                let mut data: Vec<u8> = Vec::new();
                data.reserve(fmeta.len() as usize);
                f.read_to_end(&mut data)?;
                Ok(data)
            }
        }
    }
}

fn create_app<'a, 'b>() -> clap::App<'a, 'b> {
    clap::App::new("datagenerator")
        .about("data generator")
        .arg(
            clap::Arg::with_name("output")
                .short("o")
                .long("output")
                .help("data destination(default is stdout)")
        )
        .arg(
            clap::Arg::with_name("input-file")
                .short("f")
                .long("--file")
                .help("data from file('-' is stdin)")
        )
        .arg(
            clap::Arg::with_name("input-hex")
                .short("x")
                .long("--hex")
                .help("data from hex string('0x' MUST NOT BE ADDED)")
                .conflicts_with("input-file")
        )
        .arg(
            clap::Arg::with_name("input-base64")
                .short("b")
                .long("base64")
                .help("data from base64 string")
                .conflicts_with("input-file")
                .conflicts_with("input-hex")
        )
        .arg(
            clap::Arg::with_name("separator")
                .short("s")
                .long("separator")
                .help("separator hex string(default is empty)")
        )
        .arg(
            clap::Arg::with_name("count")
                .short("c")
                .long("count")
                .help("repeat count")
                .required(true)
        )
}
fn main() {
    let mut app = create_app();
    let m = app.get_matches();
    
}
