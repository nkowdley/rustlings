// Make this compile-- start by filling in the type where the question marks are!
// Scroll down for hints :)

use std::error;
use std::fmt;
use std::io;

// So many things could go wrong!
//
// Reading from stdin could produce an io::Error
// Parsing the input could produce a num::ParseIntError
// Validating the input could produce a CreationError
//
// How can we lump these errors into one general error?

fn read_and_validate(b: &mut io::BufRead) -> Result<PositiveNonzeroInteger, ???> {
    let mut line = String::new();
    b.read_line(&mut line);
    let num: i64 = line.parse();
    PositiveNonzeroInteger::new(num)
}

/*
fn read_and_validate(b: &mut io::BufRead) -> Result<PositiveNonzeroInteger, Box<error::Error>> {
    let mut line = String::new();
    try!(b.read_line(&mut line));
    let num: i64 = try!(line.trim().parse());
    let result = try!(PositiveNonzeroInteger::new(num));
    Ok(result)
}
*/

fn test_with_str(s: &str) -> Result<PositiveNonzeroInteger, Box<error::Error>> {
    let mut b = io::BufReader::new(s.as_bytes());
    read_and_validate(&mut b)
}

#[test]
fn test_success() {
    let x = test_with_str("42\n");
    assert!(x.is_ok());
    assert_eq!(PositiveNonzeroInteger(42), x.unwrap());
}

#[test]
fn test_not_num() {
    let x = test_with_str("eleven billion\n");
    assert!(x.is_err());
}

#[test]
fn test_non_positive() {
    let x = test_with_str("-40\n");
    assert!(x.is_err());
}

#[test]
fn test_ioerror() {
    struct Broken;
    impl io::Read for Broken {
        fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::BrokenPipe, "uh-oh!"))
        }
    }
    let mut b = io::BufReader::new(Broken);
    assert!(read_and_validate(&mut b).is_err());
}

// Sprinkle the try! macro around any Result<> values,
// which under the hood calls From::from
// on the error value to convert it to a
// boxed trait object, a Box<error::Error>,
// which is polymorphic.

#[derive(PartialEq,Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq,Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str((self as &error::Error).description())
    }
}

impl error::Error for CreationError {
    fn description(&self) -> &str {
        match *self {
            CreationError::Negative => "Negative",
            CreationError::Zero => "Zero",
        }
    }
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        if value == 0 {
            Err(CreationError::Zero)
        } else if value < 0 {
            Err(CreationError::Negative)
        } else {
            Ok(PositiveNonzeroInteger(value as u64))
        }
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(Err(CreationError::Negative), PositiveNonzeroInteger::new(-10));
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
