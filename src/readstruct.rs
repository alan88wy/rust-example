// Example of reading from struct
//
//  Here is a function that can read a struct (of a POD type) from a file:

use std::io::{self, Read};
use std::slice;

fn read_struct<T, R: Read>(mut read: R) -> io::Result<T> {
    let num_bytes = ::std::mem::size_of::<T>();
    unsafe {
        let mut s = ::std::mem::uninitialized();
        let buffer = slice::from_raw_parts_mut(&mut s as *mut T as *mut u8, num_bytes);
        match read.read_exact(buffer) {
            Ok(()) => Ok(s),
            Err(e) => {
                ::std::mem::forget(s);
                Err(e)
            }
        }
    }
}

// use
// read_struct::<Configuration>(reader)

// If you want to read a sequence of structs from a file,
// you can execute read_struct multiple times or read all the file at once:

use std::fs::{self, File};
use std::io::BufReader;
use std::path::Path;

fn read_structs<T, P: AsRef<Path>>(path: P) -> io::Result<Vec<T>> {
    let path = path.as_ref();
    let struct_size = ::std::mem::size_of::<T>();
    let num_bytes = fs::metadata(path)?.len() as usize;
    let num_structs = num_bytes / struct_size;
    let mut reader = BufReader::new(File::open(path)?);
    let mut r = Vec::<T>::with_capacity(num_structs);
    unsafe {
        let buffer = slice::from_raw_parts_mut(r.as_mut_ptr() as *mut u8, num_bytes);
        reader.read_exact(buffer)?;
        r.set_len(num_structs);
    }
    Ok(r)
}

// use
// read_structs::<StructName, _>("path/to/file"))
