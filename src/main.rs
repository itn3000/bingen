use std::io::Write;
use std::io::Read;
use thiserror::Error;
use rand::RngCore;

#[derive(Error, Debug)]
enum AppError {
    #[error("io error")]
    Io(#[from]std::io::Error),
    #[error("hex parse error")]
    HexParse(#[from]hex::FromHexError),
    #[error("base64 parse error({0:?})")]
    Base64Parse(#[from]base64::DecodeError),
    #[error("decimal parse error")]
    DecimalParseError(#[from] std::num::ParseIntError),
    #[error("random value error")]
    RandomError(#[from] rand::Error),
    #[error("unknown input error")]
    UnknownInputError,
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

#[derive(Debug)]
enum InputDataType {
    Hex(String),
    Base64(String),
    File(String),
    U8String(String),
    Sin(usize),
    None
}

impl InputDataType {
    fn get_bytes(&self) -> Result<Vec<u8>, AppError>  {
        match self {
            Self::Hex(s) => Ok(hex::decode(s)?),
            Self::Base64(s) => Ok(base64::decode(s)?),
            Self::File(s) => {
                let mut f = std::fs::File::open(s)?;
                let fmeta = f.metadata()?;
                let mut data: Vec<u8> = Vec::new();
                data.reserve(fmeta.len() as usize);
                f.read_to_end(&mut data)?;
                Ok(data)
            },
            Self::U8String(s) => {
                let mut data = Vec::<u8>::new();
                data.extend(s.as_bytes());
                Ok(data)
            },
            Self::Sin(max) => {
                let max = *max;
                let mut sin = std::io::stdin();
                let mut data = Vec::<u8>::new();
                data.resize(max, 0);
                let mut offset = 0;
                let mut total = 0usize;
                loop {
                    let bytesread = sin.read(&mut data[offset..])?;
                    total += bytesread;
                    if bytesread == 0 || total >= max {
                        break;
                    }
                    offset += bytesread;
                }
                data.resize(std::cmp::min(total, max), 0);
                Ok(data)
            }
            Self::None => {
                Err(AppError::UnknownInputError)
            }
        }
    }
}
#[derive(Debug)]
struct InputData {
    input_data_type: InputDataType,
    cached: Option<Vec<u8>>,
    is_random: bool,
    rnd: rand::rngs::ThreadRng,
}
impl InputData {
    fn get_bytes(&mut self, max_length: usize) -> Result<Vec<u8>, AppError> {
        if self.is_random {
            let mut data = Vec::<u8>::new();
            data.resize(max_length, 0);
            self.rnd.fill_bytes(&mut data);
            return Ok(data);
        }
        if let Some(cached) = self.cached.as_ref() {
            return Ok(cached.clone())
        }
        let result = self.input_data_type.get_bytes()?;
        self.cached = Some(result.clone());
        Ok(result)
    }
}

fn get_outputstream(matches: &clap::ArgMatches) -> Result<StdoutOrFile, AppError> {
    match matches.value_of("output") {
        Some(p) => Ok(StdoutOrFile::File(std::fs::File::create(p)?)),
        None => Ok(StdoutOrFile::Stdout(std::io::stdout()))
    }
}

fn get_inputvalue(matches: &clap::ArgMatches) -> Result<InputData, AppError> {
    let inputdata = if let Some(b64) = matches.value_of("input-base64") {
        InputDataType::Base64(b64.to_owned())
    } else if let Some(hex) = matches.value_of("input-hex") {
        InputDataType::Hex(hex.to_owned())
    } else if let Some(f) = matches.value_of("input-file") {
        InputDataType::File(f.to_owned())
    } else if let Some(s) = matches.value_of("input-string") {
        InputDataType::U8String(s.to_owned())
    } else if matches.is_present("input-random") {
        InputDataType::None
    } else if let Some(s) = matches.value_of("input-stdin") {
        InputDataType::Sin(s.parse::<usize>()?)
    } else {
        return Err(AppError::UnknownInputError)
    };
    Ok(InputData {
        cached: None,
        input_data_type: inputdata,
        is_random: matches.is_present("input-random"),
        rnd: rand::thread_rng()
    })
}

fn create_app<'a, 'b>() -> clap::App<'a, 'b> {
    clap::App::new("datagenerator")
        .about("data generator")
        .arg(
            clap::Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("OUTPUT")
                .help("data destination(default is stdout)")
        )
        .group(
            clap::ArgGroup::with_name("input")
                .args(&["input-file", "input-base64", "input-hex", "input-string", "input-random", "input-stdin"])
                .required(true)
        )
        .arg(
            clap::Arg::with_name("input-file")
                .short("f")
                .long("--file")
                .value_name("INPUT")
                .help("data from file")
                .conflicts_with_all(&["input-hex", "input-base64", "input-string", "--input-random"])
        )
        .arg(
            clap::Arg::with_name("input-hex")
                .short("x")
                .long("--hex")
                .help("data from hex string('0x' MUST NOT BE ADDED)")
                .value_name("HEX_STRING")
                .conflicts_with_all(&["input-file", "input-base64", "input-string", "--input-random"])
        )
        .arg(
            clap::Arg::with_name("input-base64")
                .short("b")
                .long("base64")
                .help("data from base64 string")
                .value_name("BASE64_STRING")
                .conflicts_with_all(&["input-file", "input-hex", "input-string", "--input-random"])
        )
        .arg(
            clap::Arg::with_name("input-string")
                .short("s")
                .long("string")
                .help("data from string(encoding to utf-8)")
                .value_name("INPUT_STRING")
                .conflicts_with("input-file")
                .conflicts_with("input-hex")
                .conflicts_with("input-base64")
                .conflicts_with("input-random")
        )
        .arg(
            clap::Arg::with_name("input-random")
                .short("r")
                .long("random")
                .help("input random value")
        )
        .arg(
            clap::Arg::with_name("input-stdin")
                .short("i")
                .long("stdin")
                .value_name("MAX_LENGTH")
                .help("input from standard input")
        )
        .arg(
            clap::Arg::with_name("delimiter")
                .short("d")
                .long("delimiter")
                .value_name("HEX_STRING")
                .help("delimiter hex string(default is empty)")
        )
        .arg(
            clap::Arg::with_name("count")
                .short("c")
                .long("count")
                .help("repeat count")
                .value_name("COUNT")
                .required(true)
        )
}
fn get_delimiter(matches: &clap::ArgMatches) -> Result<Vec<u8>, AppError> {
    if let Some(s) = matches.value_of("delimiter") {
        Ok(hex::decode(s)?)
    } else {
        Ok(Vec::<u8>::new())
    }
}
fn get_count(matches: &clap::ArgMatches) -> Result<u64, AppError> {
    if let Some(s) = matches.value_of("count") {
        Ok(s.parse::<u64>()?)
    } else {
        Ok(1)
    }
}
fn main() -> Result<(), AppError> {
    let app = create_app();
    let m = app.get_matches();
    let mut inputdata = get_inputvalue(&m)?;
    let mut output = get_outputstream(&m)?;
    let delimiter = get_delimiter(&m)?;
    let count = get_count(&m)?;
    let mut buf = Vec::<u8>::new();
    buf.reserve(4096);
    for i in 0..count {
        buf.extend(inputdata.get_bytes(1)?.iter());
        if i + 1 < count {
            buf.extend(delimiter.iter());
        }
        if buf.len() >= 4096 {
            output.write(&buf)?;
            buf.clear();
        }
        // output.write(&bytes)?;
    }
    if buf.len() != 0 {
        output.write(&buf)?;
    }
    Ok(())
}
