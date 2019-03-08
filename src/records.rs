use std::fs::File;
use std::mem;
use std::io::{Read, Result};

use libc::{c_int, c_char};

#[derive(Debug)]
pub struct OSParams {
    /**
    Easier to work with representation of the C struct OSParamsInternal
    */
    mem_model: MemModel,
    pro_algorithm: Algorithm,
    mem_fix_blksize: i32,               /* F: Number of bytes per block  */
    mem_fix_blksaveal: i32,             /* F: Number of available blocks */
    mem_var_maxsize: i32,               /* V: Maximum block size         */
    mem_var_totsize: i32,               /* V: Total available memory     */
    pro_max_tasks: i32,                 /* Maximum Processes allowed     */
    init_quantum: i32,                  /* Default quantum size          */
    disk_units: i32,                    /* Number of disk units avail.   */
    tape_units: i32,                    /* Number of tape units avail.   */
    cdrom_units: i32,                   /* Number of CDROM units avail.  */
    every_n_units: bool,                /* If not zero, print #3 detail  */
                                        /*   output every n units        */
}
/** Enums for OSParams */
#[derive(Debug)]
enum MemModel { None, Fixed, Variable }
#[derive(Debug)]
enum Algorithm { FIFO, IPRI, MLFQ}      /* FIFO : first-in, first-out    */
                                        /* IPRI : initial priority       */
                                        /* MLFQ : multi-level fb queue   */

/** Utility function for converting 8 byte c_char arrays to str */
fn convert_bytes(buf: &[c_char; 8]) -> String {
    let mut value = String::new();
    for chr in buf.iter() {
        value.push(*chr as u8 as char);
    }
    value
}

impl OSParams {
    pub fn read_from_file(filename: &str) -> Result<OSParams> {

        //internal model of c struct read from .DAT file
        #[repr(C)]
        #[repr(packed)]
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
            pro_algorithm: [c_char; 8]
        }

        let mut file = File::open(filename)?;
        let mut data: [u8; 52] = [0; 52];
        file.read_exact(&mut data)?;
        let params: OSParamsInternal = unsafe { mem::transmute(data) };

        // parse the c struct values into rust struct
        Ok(OSParams {
            // auto convert c_int -> i32
            mem_fix_blksize: params.mem_fix_blksize,
            mem_fix_blksaveal: params.mem_fix_blksaveal,
            mem_var_maxsize: params.mem_var_maxsize,
            mem_var_totsize: params.mem_var_totsize,
            pro_max_tasks: params.pro_max_tasks,
            init_quantum: params.init_quantum,
            disk_units: params.disk_units,
            tape_units: params.tape_units,
            cdrom_units: params.cdrom_units,
            // match other values
            mem_model: match params.mem_model {
                0 => MemModel::None,
                1 => MemModel::Fixed,
                2 => MemModel::Variable,
                _ => MemModel::None
            },
            pro_algorithm: match convert_bytes(&params.pro_algorithm).as_str() {
                "FIFO" => Algorithm::FIFO,
                "IPRI" => Algorithm::IPRI,
                "MLFQ" => Algorithm::MLFQ,
                _ => Algorithm::FIFO
            },
            every_n_units: match params.every_n_units {
                0 => false,
                _ => true
            }
        })
    }
}
impl InputDataInternal {
    fn read_from_file(filename: &str) -> Result<InputDataInternal> {
        let mut file = File::open(filename)?;
        let mut data: [u8; 136] = [0; 136];

        file.read_exact(&mut data)?;
        let inp: InputDataInternal = unsafe { mem::transmute(data) };
        Ok(inp)
    }
}



#[repr(C)]
#[repr(packed)]
pub struct InputDataInternal {
    process_priority: c_int,            /* User assigned priority        */
    process_memsize: c_int,             /* Load module memory requirement*/
    run_info: [[c_int; 3]; 10],         /* 10 groups of 3 integers:      */
    /*    0 = CPU units              */
    /*    1 = I/O units              */
    /*    2 = I/O device types:      */
    /*        1 = DEV_DISK           */
    /*        2 = DEV_TAPE           */
    /*        3 = DEV_CD             */
    /*  0 thru 9 is the 10 cycles    */
    process_name: [c_char; 8]           /* User name of process 7 chars  */
}
