use std::{io, path, fs};

fn copy_dir<T, U>(from: T, to: U) -> Result<(), io::Error> 
where
    T: AsRef<path::Path>,
    U: AsRef<path::Path>,
{
    let src = from.as_ref();
    let dst = to.as_ref();

    if !dst.exists() {
        fs::create_dir(&dst);
    }

    for de in fs::read_dir(src)?.into_iter() {
        let entry = &de?;
        let fname = entry.file_name();
        let name = fname.to_str().unwrap();
        let p = entry.path();
        if p.is_dir() {
            copy_dir(&p, &dst.join(name));
        } else {
            fs::copy(&p, &dst.join(name));
        };
    }

    Ok(())
}

fn main() {
}
