// extern crate image;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
  let mut entries = fs::read_dir(".")?
    .map(|res| res.map(|e| e.path()))
    .collect::<Result<Vec<_>, io::Error>>()?;

  println!("{:?}", entries);

  Ok(())
}
