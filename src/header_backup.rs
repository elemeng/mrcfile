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

#[derive(Debug)]
struct MRCHeader {
    nx: i32,
    ny: i32,
    nz: i32,
    mode: i32,
    nxstart: i32,
    nystart: i32,
    nzstart: i32,
    mx: i32,
    my: i32,
    mz: i32,
    cella: [f32; 3],
    cellb: [f32; 3],
    mapc: i32,
    mapr: i32,
    maps: i32,
    dmin: f32,
    dmax: f32,
    dmean: f32,
    ispg: i32,
    nsymbt: i32,
    extra: [u8; 100],
    exttyp: i32,
    nversion: i32,
    origin: [f32; 3],
    map_id: String,
    machst: [u8; 4],
    rms: f32,
    nlabl: i32,
    labels: [String; 10],
}

impl MRCHeader {
    fn new() -> Self {
        MRCHeader {
            nx: 0,
            ny: 0,
            nz: 0,
            mode: 0,
            nxstart: 0,
            nystart: 0,
            nzstart: 0,
            mx: 0,
            my: 0,
            mz: 0,
            cella: [0.0; 3],
            cellb: [0.0; 3],
            mapc: 0,
            mapr: 0,
            maps: 0,
            dmin: 0.0,
            dmax: 0.0,
            dmean: 0.0,
            ispg: 0,
            nsymbt: 0,
            extra: [0; 100],
            exttyp: 0,
            nversion: 0,
            origin: [0.0; 3],
            map_id: String::new(),
            machst: [0; 4],
            rms: 0.0,
            nlabl: 0,
            labels: [String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new()],
        }
    }

    fn read_from_file(file_path: &str) -> io::Result<Self> {
        let mut file = File::open(file_path)?;
        let mut header = MRCHeader::new();

        header.nx = file.read_i32::<LittleEndian>()?;
        header.ny = file.read_i32::<LittleEndian>()?;
        header.nz = file.read_i32::<LittleEndian>()?;
        header.mode = file.read_i32::<LittleEndian>()?;
        header.nxstart = file.read_i32::<LittleEndian>()?;
        header.nystart = file.read_i32::<LittleEndian>()?;
        header.nzstart = file.read_i32::<LittleEndian>()?;
        header.mx = file.read_i32::<LittleEndian>()?;
        header.my = file.read_i32::<LittleEndian>()?;
        header.mz = file.read_i32::<LittleEndian>()?;
        
        for i in 0..3 {
            header.cella[i] = file.read_f32::<LittleEndian>()?;
        }
        
        for i in 0..3 {
            header.cellb[i] = file.read_f32::<LittleEndian>()?;
        }
        
        header.mapc = file.read_i32::<LittleEndian>()?;
        header.mapr = file.read_i32::<LittleEndian>()?;
        header.maps = file.read_i32::<LittleEndian>()?;
        header.dmin = file.read_f32::<LittleEndian>()?;
        header.dmax = file.read_f32::<LittleEndian>()?;
        header.dmean = file.read_f32::<LittleEndian>()?;
        header.ispg = file.read_i32::<LittleEndian>()?;
        header.nsymbt = file.read_i32::<LittleEndian>()?;
        
        file.read_exact(&mut header.extra)?;
        
        header.exttyp = file.read_i32::<LittleEndian>()?;
        header.nversion = file.read_i32::<LittleEndian>()?;
        
        for i in 0..3 {
            header.origin[i] = file.read_f32::<LittleEndian>()?;
        }
        
        let mut map_id_bytes = [0; 4];
        file.read_exact(&mut map_id_bytes)?;
        header.map_id = String::from_utf8_lossy(&map_id_bytes).into_owned();
        
        file.read_exact(&mut header.machst)?;
        header.rms = file.read_f32::<LittleEndian>()?;
        header.nlabl = file.read_i32::<LittleEndian>()?;
        
        for i in 0..10 {
            let mut label_bytes = [0; 80];
            file.read_exact(&mut label_bytes)?;
            header.labels[i] = String::from_utf8_lossy(&label_bytes).trim_end_matches('\0').to_owned();
        }

        Ok(header)
    }

    fn display(&self) {
        println!("NX: {}", self.nx);
        println!("NY: {}", self.ny);
        println!("NZ: {}", self.nz);
        println!("MODE: {}", self.mode);
        println!("NXSTART: {}", self.nxstart);
        println!("NYSTART: {}", self.nystart);
        println!("NZSTART: {}", self.nzstart);
        println!("MX: {}", self.mx);
        println!("MY: {}", self.my);
        println!("MZ: {}", self.mz);
        println!("CELLA: {:?}", self.cella);
        println!("CELLB: {:?}", self.cellb);
        println!("MAPC: {}", self.mapc);
        println!("MAPR: {}", self.mapr);
        println!("MAPS: {}", self.maps);
        println!("DMIN: {}", self.dmin);
        println!("DMAX: {}", self.dmax);
        println!("DMEAN: {}", self.dmean);
        println!("ISPG: {}", self.ispg);
        println!("NSYMBT: {}", self.nsymbt);
        println!("EXTTYP: {}", self.exttyp);
        println!("NVERSION: {}", self.nversion);
        println!("ORIGIN: {:?}", self.origin);
        println!("MAP ID: {}", self.map_id);
        println!("MACHST: {:?}", self.machst);
        println!("RMS: {}", self.rms);
        println!("NLABL: {}", self.nlabl);
        for (i, label) in self.labels.iter().enumerate() {
            println!("LABEL {}: {}", i + 1, label);
        }
    }
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let header = MRCHeader::read_from_file(&cli.file_path)?;
    header.display();
    Ok(())
}
