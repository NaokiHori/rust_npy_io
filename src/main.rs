use std::io::{Read, Write};

fn write_f64_data(f: &mut std::fs::File, data: &[f64]) -> Result<(), std::io::Error> {
    for datum in data {
        let bytes = datum.to_ne_bytes();
        f.write_all(&bytes)?;
    }
    Ok(())
}

fn load_f64_data(f: &mut std::fs::File, header: &rust_npy_io::Header) -> Result<Vec<f64>, String> {
    let nitems: usize = header.shape.iter().product();
    const SIZE_OF_F64: usize = std::mem::size_of::<f64>();
    let mut buf = vec![0u8; nitems * SIZE_OF_F64];
    match f.read_exact(&mut buf) {
        Ok(_) => {}
        Err(_) => return Err("read_exact failed".to_string()),
    };
    let data = buf
        .chunks_exact(SIZE_OF_F64)
        .map(|chunk| {
            let mut arr = [0u8; SIZE_OF_F64];
            arr.copy_from_slice(chunk);
            f64::from_ne_bytes(arr)
        })
        .collect();
    Ok(data)
}

fn main() {
    let file_name = "sample.npy";
    let mut f = std::fs::File::create(file_name).unwrap();
    let shape = [3usize, 5usize];
    let header = rust_npy_io::Header {
        descr: "'<f8'".to_string(),
        fortran_order: false,
        shape: shape.to_vec(),
    };
    match rust_npy_io::write_header(&mut f, &header) {
        Ok(_) => {}
        Err(e) => {
            println!("Failed to write header: {}", e);
            std::process::exit(1);
        }
    };
    let mut data = vec![0f64; shape[0] * shape[1]];
    for (n, datum) in data.iter_mut().enumerate().take(shape[0] * shape[1]) {
        *datum = n as f64;
    }
    write_f64_data(&mut f, &data).unwrap();
    let mut f = match std::fs::File::open(file_name) {
        Ok(f) => f,
        Err(_) => {
            println!("failed to open file: {}", file_name);
            std::process::exit(1);
        }
    };
    let header: rust_npy_io::Header = match rust_npy_io::read_header(&mut f) {
        Ok(header) => header,
        Err(e) => {
            println!("Failed to read header: {}", e);
            std::process::exit(1);
        }
    };
    println!("Shape: {:?}", header.shape);
    let data = match load_f64_data(&mut f, &header) {
        Ok(data) => data,
        Err(message) => {
            println!("{}", message);
            std::process::exit(1);
        }
    };
    println!("Data: {:?}", data);
}
