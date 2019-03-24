/*********************************************************/
/*** OS_SYS v0.1; Written: March 2019 By: A. Sojourner ***/
/***===================================================***/
/*** Program simulates a simple Operating System.      ***/
/*** Input Files:                                      ***/
/***   1) O/S Parameter File: OS_OSP.DAT               ***/
/***   2) Input Queue Data: OS_INP.DAT                 ***/
/*** Output Files:                                     ***/
/***   1) Print O/S Start-End times and Parameter data.***/
/***   2) Print Process Allocation... De-allocation    ***/
/***       information.                                ***/
/***   3) If Print_Every_N_Units is not 0, print       ***/
/***       detail information each N units.            ***/
/*********************************************************/

#![allow(non_snake_case)]
#![allow(dead_code)]

mod os;
mod records;

use os::os::OS;
use records::{OSParams, ProcessData};

fn main() {
    let params = open_params();
    let all_records = open_records();

    let mut os = OS::new(params, all_records);
    os.start();
}

fn open_params() -> OSParams {
    match OSParams::read_from_file("./res/OS_OSP.DAT") {
        Ok(t) => t,
        Err(e) => panic!("{}", e),
    }
}
fn open_records() -> Vec<ProcessData> {
    match ProcessData::read_from_file("./res/OS_INP.DAT", 10) {
        Ok(t) => t,
        Err(e) => panic!("{}", e),
    }
}
