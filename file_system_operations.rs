use std::fs::{self, File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::os::unix;
use std::path::Path;

fn main() {
    if let Ok(contents) = cat(&Path::new("file_system_operations.rs")) {
        println!("cat: {}", contents);
    }

    echo("Hola, Mundo!", &Path::new("hola_mundo.txt")).unwrap();
    touch(&Path::new("foo1.txt")).unwrap();

    // testing out the new APIs

    println!("mkdir a");

    match fs::create_dir("a") {
        Err(err) => println!("{:?}", err.kind()),
        Ok(_) => {}
    }

    println!("echo hello > a/b.txt");
    echo("hello", &Path::new("a/b.txt")).unwrap_or_else(|err| {
        println!("{:?}", err.kind());
    });

    println!("mkdir -p a/c/d");
    fs::create_dir_all("a/c/d").unwrap_or_else(|err| {
        println!("{:?}", err.kind());
    });

    println!("touch a/c/e.txt");
    touch(&Path::new("a/c/e,txt")).unwrap_or_else(|err| {
        println!(
            "{:?
        }",
            err.kind()
        );
    });

    println!("ln -s ../b.txt a/c/b.txt");
    if cfg!(target_family = "unix") {
        unix::fs::symlink("../b.txt", "a/c/b.txt").unwrap_or_else(|err| {
            println!("{:?}", err.kind());
        });
    }

    println!("cat a/c/b.txt");
    let contents = match cat(&Path::new("a/c/b.txt")) {
        Err(err) => panic!("{:?}", err.kind()),
        Ok(contents) => contents,
    };

    println!("{}", contents);

    match fs::read_dir("a") {
        Err(err) => println!("{:?}", err.kind()),
        Ok(paths) => {
            for path in paths {
                println!("{:?}", path);
            }
        }
    }

    println!("rm a/c/e.txt");
    fs::remove_file("a/c/e.txt").unwrap_or_else(|err| {
        println!("{:?}", err.kind());
    });

    println!("rmdir a/c/d");
    fs::remove_dir("a/c/d").unwrap_or_else(|err| {
        println!("{:?}", err.kind());
    });
}

fn cat(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(err) => Err(err),
        Ok(_) => Ok(s),
    }
}

fn echo(s: &str, path: &Path) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(s.as_bytes())
}

fn touch(path: &Path) -> io::Result<()> {
    match OpenOptions::new().create(true).write(true).open(path) {
        Err(err) => Err(err),
        Ok(_) => Ok(()),
    }
}
