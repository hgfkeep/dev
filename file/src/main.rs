#![feature(with_options)]
use std::io::{BufRead, Write};

/// 创建文件
///
/// 🤔如果文件已存在会怎么样？
fn file_info(filename: &str) -> std::io::Result<()> {
    // 简单创建文件， 如果文件存在会自动截断
    let file: std::fs::File = std::fs::File::create(filename)?;

    // 获取文件的属性信息, 文件的创建时间， 最近的访问时间，最新的修改时间，文件权限，是否是文件/文件夹
    let metadata = file.metadata()?;
    println!("{:?}", metadata);

    // 获取文件的权限信息
    let permission = metadata.permissions();
    println!("is file read only? {}", permission.readonly());

    // 文件大小
    let size = metadata.len();
    println!("file size:{:?}", size);

    // 高度定制化的文件打开方式, with_options是unstable的接口
    let mut file = std::fs::File::with_options()
        .read(true)
        .write(true)
        .open("/tmp/hgf.txt")?;
    let size = file.write(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])?; // write 操作必须引入 std::io::Write trait。
    file.flush()?; // write 完成后一定记得 刷到磁盘
    println!("write size: {}", size);

    std::fs::write("/tmp/x", b"this is a byte\nse\n thrid line")?;

    let mut file = std::fs::File::with_options()
        .create(true)
        .write(true)
        .open("/tmp/aaa")?; // c
    let size = file.write(b"this is a auto create file test")?;
    file.flush()?;
    println!("write size: {}", size);

    // std::fs::create_dir("/etc/hgf")?; //Permission denied
    // std::fs::create_dir("/tmp/aaa/bbb/hgf")?; // Not a directory
    // std::fs::create_dir("/tmp")?;  //File exists

    let path = std::path::Path::new("/tmp");
    for entry in path.read_dir().expect("not a dir") {
        if let Ok(entry) = entry {
            println!(
                "dir: name={:?}, type={:?}, path={:?} ",
                entry.file_name(),
                entry.file_type(), // 通过file_type 可以判断是否是文件
                entry.path().as_os_str()
            );
        }
    }

    let file = std::fs::File::open("/tmp/x")?;
    let mut bufReader = std::io::BufReader::new(file);
    for line in bufReader.lines() {
        println!("line: {}", line.unwrap());
    }
    // let mut s: String = String::new();
    // while let Ok(n) = bufReader.read_line(&mut s) {
    //     if n==0 {
    //         break;
    //     }
    //     println!("line: {}, size:{}", s, n);
    //     s.clear();

    // }

    Ok(())
}

fn gzip() -> std::io::Result<()> {
    let compressed_file = std::fs::File::create("xx.tar.gz")?;
    let mut encoder =
        flate2::write::GzEncoder::new(compressed_file, flate2::Compression::default());
    {
        let mut achive = tar::Builder::new(&mut encoder); // tar 文件会将数据写到底层的encoder上面去，即gz压缩。
        achive.append_dir_all("tmp", "/tmp/xxx")?; // 将/tmp/xxx 文件夹压缩到压缩文件中， 文件夹名字改为tmp
    }
    encoder.finish()?;

    Ok(())
}

fn main() {
    // file_info("/tmp/hgf.txt").unwrap();
    // std::fs::rename("/tmp/y", "/tmp/xxx/y").unwrap();
    gzip().unwrap();
}
