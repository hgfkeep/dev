use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, Cursor, SeekFrom};


pub fn write_last_ten_bytese<W: Write + Seek>(writer: &mut W) -> std::io::Result<()> {
    writer.seek(SeekFrom::End(-10))?;
    for i in 0..10 {
        writer.write(&[i])?;
    }
    Ok(())
}

pub fn testFile() -> io::Result<()> {
    println!("flush to file");
    let mut file = File::create("file.txt")?;
    file.write_all("hgfgood!!!!".as_bytes())?;
    write_last_ten_bytese(&mut file)?;
    file.flush()?;
    println!("flushed");
    
    let mut file = File::open("file.txt")?;
    file.seek(SeekFrom::Start(0))?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    println!("file content: {:?}", buf);

    Ok(())
}

pub fn testBuf() -> io::Result<()> {
    let mut buf = Cursor::new(vec![0; 15]);
    write_last_ten_bytese(&mut buf)?;
    println!("After write: {:?}", buf);
    Ok(())
}
