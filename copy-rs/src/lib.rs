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

#[cfg(test)]
mod tests {
    use crate::{copy_dir};

    use std::{env, fs};
    use rand::{thread_rng, Rng};

    fn get_random_name(length: i32) -> String {
        let mut s = String::new();
        for i in 0..length {
            let n = thread_rng().gen_range(65..91);
            s.push(char::from_u32(n).unwrap());
        }
        s
    }

    #[test]
    fn test_copying_file_to_other_directory() {
        let tmp_dir = env::temp_dir();

        let src = tmp_dir.join(get_random_name(5));
        let dst = tmp_dir.join(get_random_name(5));

        fs::create_dir(&src);
        let fname = get_random_name(6);
        fs::write(&src.join(&fname), "Hello World");


        copy_dir(&src, &dst);

        let actual = fs::read_to_string(&dst.join(&fname)).unwrap();
        assert_eq!("Hello World", actual);
    }

    #[test]
    fn test_copying_file_inside_nested_directory() {
        let tmp_dir = env::temp_dir();

        let src = tmp_dir.join(get_random_name(5));
        let dst = tmp_dir.join(get_random_name(5));

        fs::create_dir(&src);
        fs::create_dir(&src.join("nested"));

        let fname = get_random_name(6);
        fs::write(&src.join("nested").join(&fname), "Hello World");

        copy_dir(&src, &dst);

        let actual = fs::read_to_string(&dst.join("nested").join(&fname)).unwrap();
        assert_eq!("Hello World", actual);
    }

    #[test]
    fn test_copying_files_and_nested_folders() {
        let tmp_dir = env::temp_dir();

        let src = tmp_dir.join(get_random_name(5));
        let dst = tmp_dir.join(get_random_name(5));

        fs::create_dir(&src);
        fs::create_dir(&src.join("nested"));

        let fname = get_random_name(6);
        fs::write(&src.join(&fname), "Hello world!");

        let nested_fname = get_random_name(6);
        fs::write(&src.join("nested").join(&nested_fname), "Hello World");

        copy_dir(&src, &dst);

        let nested_actual = fs::read_to_string(&dst.join("nested").join(&nested_fname)).unwrap();
        let actual = fs::read_to_string(&dst.join(&fname)).unwrap();

        assert_eq!("Hello world!", actual);
        assert_eq!("Hello World", nested_actual);
    }
}
