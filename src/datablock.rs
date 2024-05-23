use std::fs::File;
use std::io::{self, Read};
use byteorder::{LittleEndian, ReadBytesExt};
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the MRC file
    file_path: String,
}

struct MRCHeader {
    // Header fields (same as before)
}

impl MRCHeader {
    // Methods for reading header and displaying header (same as before)
}

fn read_data_block(file: &mut File, header: &MRCHeader) -> io::Result<Vec<Vec<Vec<f32>>>> {
    let nx = header.nx as usize;
    let ny = header.ny as usize;
    let nz = header.nz as usize;
    let mode = header.mode;

    let mut data = vec![vec![vec![0.0; nz]; ny]; nx];

    match mode {
        0 => read_data_as_i8(file, &mut data, nx, ny, nz)?,
        1 => read_data_as_i16(file, &mut data, nx, ny, nz)?,
        2 => read_data_as_i32(file, &mut data, nx, ny, nz)?,
        3 => read_data_as_complex_i16(file, &mut data, nx, ny, nz)?,
        4 => read_data_as_complex_i32(file, &mut data, nx, ny, nz)?,
        5 => read_data_as_u16(file, &mut data, nx, ny, nz)?,
        6 => read_data_as_f16(file, &mut data, nx, ny, nz)?,
        101 => read_data_as_4bit_packed(file, &mut data, nx, ny, nz)?,
        _ => return Err(io::Error::new(io::ErrorKind::Other, "Unsupported data mode")),
    }

    Ok(data)
}

fn read_data_as_i8(file: &mut File, data: &mut Vec<Vec<Vec<f32>>>, nx: usize, ny: usize, nz: usize) -> io::Result<()> {
    for k in 0..nz {
        for j in 0..ny {
            for i in 0..nx {
                let value = file.read_i8()? as f32;
                data[i][j][k] = value;
            }
        }
    }
    Ok(())
}

fn read_data_as_i16(file: &mut File, data: &mut Vec<Vec<Vec<f32>>>, nx: usize, ny: usize, nz: usize) -> io::Result<()> {
    for k in 0..nz {
        for j in 0..ny {
            for i in 0..nx {
                let value = file.read_i16::<LittleEndian>()? as f32;
                data[i][j][k] = value;
            }
        }
    }
    Ok(())
}

fn read_data_as_i32(file: &mut File, data: &mut Vec<Vec<Vec<f32>>>, nx: usize, ny: usize, nz: usize) -> io::Result<()> {
    for k in 0..nz {
        for j in 0..ny {
            for i in 0..nx {
                let value = file.read_i32::<LittleEndian>()? as f32;
                data[i][j][k] = value;
            }
        }
    }
    Ok(())
}

fn read_data_as_complex_i16(file: &mut File, data: &mut Vec<Vec<Vec<f32>>>, nx: usize, ny: usize, nz: usize) -> io::Result<()> {
    // Handle complex data type (mode 3)
    // Each value consists of two consecutive 16-bit signed integers (real and imaginary parts)
    for k in 0..nz {
        for j in 0..ny {
            for i in 0..nx {
                let real = file.read_i16::<LittleEndian>()? as f32;
                let imag = file.read_i16::<LittleEndian>()? as f32;
                data[i][j][k] = real + imag * 1.0j;
            }
        }
    }
    Ok(())
}

fn read_data_as_complex_i32(file: &mut File, data: &mut Vec<Vec<Vec<f32>>>, nx: usize, ny: usize, nz: usize) -> io::Result<()> {
    // Handle complex data type (mode 4)
    // Each value consists of two consecutive 32-bit signed integers (real and imaginary parts)
    for k in 0..nz {
        for j in 0..ny {
            for i in 0..nx {
                let real = file.read_i32::<LittleEndian>()? as f32;
                let imag = file.read_i32::<LittleEndian>()? as f32;
                data[i][j][k] = real + imag * 1.0j;
            }
        }
    }
    Ok(())
}

fn read_data_as_u16(file: &mut File, data: &mut Vec<Vec<Vec<f32>>>, nx: usize, ny: usize, nz: usize) -> io::Result<()> {
    // Handle unsigned 16-bit integer data (mode 5)
    for k in 0..nz {
        for j in 0..ny {
            for i in 0..nx {
                let value = file.read_u16::<LittleEndian>()? as f32;
                data[i][j][k] = value;
            }
        }
    }
    Ok(())
}

fn read_data_as_f16(file: &mut File, data: &mut Vec<Vec<Vec<f32>>>, nx: usize, ny: usize, nz: usize) -> io::Result<()> {
    // Handle 16-bit floating point data (mode 6)
    // Note: Half-precision floating-point (f16) is not directly supported in Rust's standard library,
    // so you may need to implement a custom conversion function if needed.
    for k in 0..nz {
        for j in 0..ny {
            for i in 0..nx {
                // Example: Read two bytes and convert to f32
                let bytes: [u8; 2] = file.read_u16::<LittleEndian>()?.to_le_bytes();
                let value = f16::from_bits(u16::from_le_bytes(bytes)).to_f32();
                data[i][j][k] = value;
            }
        }
    }
    Ok(())
}

fn read_data_as_4bit_packed(file: &mut File, data: &mut Vec<Vec<Vec<f32>>>, nx: usize, ny: usize, nz: usize) -> io::Result<()> {
    // Handle 4-bit data packed two per byte (mode 101)
    for k in 0..nz {
        for j in 0..ny {
            let mut byte = 0u8;
            let mut bit_pos = 0;
            for i in 0..nx {
                if bit_pos == 0 {
                    byte = file.read_u8()?;
                }
                let value = ((byte >> (bit_pos * 4)) & 0xF) as f32;
                data[i][j][k] = value;
                bit_pos = (bit_pos + 1) % 2;
            }
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let mut file = File::open(&cli.file_path)?;
    let header = MRCHeader::read_from_file(&cli.file_path)?;

    let data = read_data_block(&mut file, &header)?;

    // Process or analyze the data as needed

    Ok(())
}
