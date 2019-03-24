use libc::{c_char, c_int};
use std::fs::File;
use std::io::{Read, Result, Seek, SeekFrom};
use std::mem;

use cute::c;

/** Program input params */
#[derive(Debug, Clone)]
pub struct OSParams {
    pub mem_model: MemModel,
    pub pro_algorithm: Algorithm,
    pub mem_fix_block_size: i32,   /* F: Number of bytes per block  */
    pub mem_fix_total_blocks: i32, /* F: Number of available blocks */
    pub mem_var_maxsize: i32,      /* V: Maximum block size         */
    pub mem_var_totsize: i32,      /* V: Total available memory     */
    pub pro_max_tasks: i32,        /* Maximum Processes allowed     */
    pub init_quantum: i32,         /* Default quantum size          */
    pub disk_units: i32,           /* Number of disk units avail.   */
    pub tape_units: i32,           /* Number of tape units avail.   */
    pub cdrom_units: i32,          /* Number of CDROM units avail.  */
    pub every_n_units: i32,        /* If not zero, print #3 detail  */
                                   /*   output every n units        */
}
/** Enums for OSParams */
#[derive(Debug, Clone)]
pub enum MemModel {
    None,
    Fixed,
    Variable,
    Unknown,
}
#[derive(Debug, Clone)]
pub enum Algorithm {
    FIFO,
    IPRI,
    MLFQ,
    Unknown,
}
/* FIFO : first-in, first-out    */
/* IPRI : initial priority       */
/* MLFQ : multi-level fb queue   */

/**
Input info for a single process
*/
#[derive(Debug, Clone)]
pub struct ProcessData {
    pub process_priority: i32,  /* User assigned priority        */
    pub process_memsize: i32,   /* Load module memory requirement*/
    pub run_info: Vec<RunInfo>, /* Cycles of process run info     */
    pub process_name: String,   /* User name of process 7 chars  */
}
/**
Info for each 'cycle' of the running process
*/
#[derive(Debug, Clone)]
pub struct RunInfo {
    pub CPU_units: i32,
    pub IO_units: i32,
    pub IO_device_type: IODeviceType,
}
#[derive(Debug, Clone)]
pub enum IODeviceType {
    Disk,
    Tape,
    CD,
    Unknown,
}

/** Utility function for converting 8 byte c_char arrays to str */
fn convert_bytes(buf: &[c_char; 8]) -> String {
    let mut value = String::new();
    for chr in buf.iter() {
        let chr = *chr as u8 as char;
        if chr != '\u{0}' {
            // ignore c-string newline
            value.push(chr);
        }
    }
    value
}

impl OSParams {
    pub fn read_from_file(filename: &str) -> Result<OSParams> {
        //internal model of c struct read from .DAT file
        #[repr(C)]
        struct OSParamsInternal {
            mem_model: c_int,
            mem_fix_blksize: c_int,
            mem_fix_blksaveal: c_int,
            mem_var_maxsize: c_int,
            mem_var_totsize: c_int,
            pro_max_tasks: c_int,
            init_quantum: c_int,
            disk_units: c_int,
            tape_units: c_int,
            cdrom_units: c_int,
            every_n_units: c_int,
            pro_algorithm: [c_char; 8],
        }

        let mut file = File::open(filename)?;
        let mut data: [u8; 52] = [0; 52];
        file.read_exact(&mut data)?;
        let params: OSParamsInternal = unsafe { mem::transmute(data) };

        // parse the c struct values into rust struct
        Ok(OSParams {
            // auto convert c_int -> i32
            mem_fix_block_size: params.mem_fix_blksize,
            mem_fix_total_blocks: params.mem_fix_blksaveal,
            mem_var_maxsize: params.mem_var_maxsize,
            mem_var_totsize: params.mem_var_totsize,
            pro_max_tasks: params.pro_max_tasks,
            init_quantum: params.init_quantum,
            disk_units: params.disk_units,
            tape_units: params.tape_units,
            cdrom_units: params.cdrom_units,
            every_n_units: params.every_n_units,
            // match other values
            mem_model: match params.mem_model {
                0 => MemModel::None,
                1 => MemModel::Fixed,
                2 => MemModel::Variable,
                _ => MemModel::Unknown,
            },
            pro_algorithm: match convert_bytes(&params.pro_algorithm).as_str() {
                "FIFO" => Algorithm::FIFO,
                "IPRI" => Algorithm::IPRI,
                "MLFQ" => Algorithm::MLFQ,
                _ => Algorithm::Unknown,
            },
        })
    }
}
impl ProcessData {
    pub fn read_from_file(filename: &str, num_entries: u32) -> Result<Vec<ProcessData>> {
        let mut file = File::open(filename)?;
        let mut data: Vec<ProcessData> = Vec::new();
        for i in 1..num_entries + 1 {
            data.push(ProcessData::read_one_entry(&mut file)?);
            file.seek(SeekFrom::Start((i * 136) as u64))?;
        }
        Ok(data)
    }
    fn read_one_entry(file: &mut File) -> Result<ProcessData> {
        #[repr(C)]
        struct InputDataInternal {
            process_priority: c_int,
            process_memsize: c_int,
            run_info: [[c_int; 3]; 10],
            process_name: [c_char; 8],
        }
        // the size of the C struct InputDataInternal in bytes _should_ be 136.
        assert_eq!(136, mem::size_of::<InputDataInternal>());

        let mut data: [u8; 136] = [0; 136];
        file.read_exact(&mut data)?;
        let inp: InputDataInternal = unsafe { mem::transmute(data) };
        Ok(ProcessData {
            process_priority: inp.process_priority,
            process_memsize: inp.process_memsize,
            process_name: convert_bytes(&inp.process_name),
            // use cute array comprehension crate to build up RunInfo vec
            run_info: c![
                RunInfo {
                    CPU_units: info[0],
                    IO_units: info[1],
                    IO_device_type: match info[2] {
                    1 => IODeviceType::Disk,
                    2 => IODeviceType::Tape,
                    3 => IODeviceType::CD,
                    _ => IODeviceType::Unknown
                }
            }, for info in inp.run_info.iter()
                // filter run info for only entries that exist; don't insert if all fields are empty
                .filter(|info| !info.iter().all(|x| *x == 0))
            ],
        })
    }
}
