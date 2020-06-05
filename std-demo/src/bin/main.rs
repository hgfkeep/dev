extern crate std_demo;
use std_demo::cursor::testBuf;
use std_demo::cursor::testFile;
fn main() -> std::io::Result<()> {
    testFile()?;
    // testBuf();

    Ok(())
}
