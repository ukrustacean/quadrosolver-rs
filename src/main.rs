use std::fmt::{Display, Formatter};
use std::io::{BufRead, Write};
use std::process::exit;

#[derive(Debug)]
enum FileModeError {
    FileDoesNotExist { filename: String },
    InvalidFileFormat,
    FirstCoefficientIsZero,
}

impl<S: AsRef<str>> From<S> for FileModeError {
    fn from(value: S) -> Self {
        use FileModeError::*;
        let s = value.as_ref();
        FileDoesNotExist {
            filename: s.to_owned(),
        }
    }
}

impl Display for FileModeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: ")?;
        match self {
            FileModeError::FileDoesNotExist { filename } => {
                write!(f, "File does not exist: {}", filename)
            }
            FileModeError::InvalidFileFormat => write!(f, "Invalid file format"),
            FileModeError::FirstCoefficientIsZero => write!(f, "First coefficient is zero"),
        }
    }
}

fn main() {
    let arg = std::env::args().skip(1).next();

    let coefs = if let Some(filename) = arg {
        match file_mode(&filename) {
            Ok(c) => c,
            Err(e) => {
                println!("{}", e);
                exit(1);
            }
        }
    } else {
        interactive_mode()
    };

    println!("{:?}", solve(coefs));
}

fn file_mode(name: &str) -> Result<[f64; 3], FileModeError> {
    use FileModeError::*;

    let file = std::fs::File::open(name).map_err(|_| FileModeError::from(name))?;
    let s = std::io::read_to_string(file).map_err(|_| InvalidFileFormat)?;
    let coefs: [f64; 3] = s
        .trim()
        .split(' ')
        .filter_map(|c| c.parse::<f64>().ok())
        .collect::<Vec<_>>()
        .try_into()
        .map_err(|_| InvalidFileFormat)?;

    if coefs[0] == 0.0 {
        Err(FirstCoefficientIsZero)?
    }

    Ok(coefs)
}

fn interactive_mode() -> [f64; 3] {
    let names = ["a", "b", "c"];
    let mut stdin = std::io::stdin().lock();
    let mut stdout = std::io::stdout().lock();
    let mut values = [0.0; 3];

    for (idx, v) in values.iter_mut().enumerate() {
        *v = loop {
            write!(stdout, "{} = ", names[idx]).unwrap();
            stdout.flush().unwrap();

            let mut input = String::new();
            stdin.read_line(&mut input).unwrap();
            let input = input.trim();

            let Ok(value) = input.parse() else {
                writeln!(stdout, "Expected a real number, got `{}`", input).unwrap();
                continue;
            };

            if idx == 0 && value == 0.0 {
                writeln!(stdout, "First coefficient can't be zero").unwrap();
                continue;
            }

            break value;
        };
    }

    values
}

fn solve([a, b, c]: [f64; 3]) -> [f64; 2] {
    let d = (b.powi(2) - 4.0 * a * c).sqrt();
    [(-b + d) / 2.0 / a, (-b - d) / 2.0 / a]
}
