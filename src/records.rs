use std::fs::File;
use std::mem;
use std::io::{Read, Result};

use libc::{c_int, c_char};


#[derive(Debug)]
pub struct OSParams {
    mem_model: c_int,                   /* 0=None, 1=Fixed, 2=Variable   */
    mem_fix_blksize: c_int,             /* F: Number of bytes per block  */
    mem_fix_blksaveal: c_int,           /* F: Number of available blocks */
    mem_var_maxsize: c_int,             /* V: Maximum block size         */
    mem_var_totsize: c_int,             /* V: Total available memory     */
    pro_max_tasks: c_int,               /* Maximum Processes allowed     */
    init_quantum: c_int,                /* Default quantum size          */
    disk_units: c_int,                  /* Number of disk units avail.   */
    tape_units: c_int,                  /* Number of tape units avail.   */
    cdrom_units: c_int,                 /* Number of CDROM units avail.  */
    every_n_units: c_int,               /* If not zero, print #3 detail  */
                                        /*   output every n units        */
    pro_algorithm: [c_char; 8]          /* FIFO : first-in, first-out    */
                                        /* IPRI : initial priority       */
                                        /* MLFQ : multi-level fb queue   */
}

#[derive(Debug)]
pub struct InputData {
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


impl OSParams {
    pub fn read_from_file(filename: &str) -> Result<OSParams> {
        let mut file = File::open(filename)?;
        let mut data: [u8; 52] = [0; 52];

        file.read_exact(&mut data)?;
        let osp: OSParams = unsafe { mem::transmute(data) };
        Ok(osp)
    }
}
impl InputData {
    pub fn read_from_file(filename: &str) -> Result<InputData> {
        let mut file = File::open(filename)?;
        let mut data: [u8; 136] = [0; 136];

        file.read_exact(&mut data)?;
        let inp: InputData = unsafe { mem::transmute(data) };
        Ok(inp)
    }
}

