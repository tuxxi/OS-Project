#![allow(non_snake_case)]
#[macro_use]
extern crate cute;

mod records;

fn main() {
    let params;
    match records::OSParams::read_from_file("./res/OS_OSP.DAT") {
        Ok(t) => params = t,
        Err(e) => panic!("{}", e)
    }

    println!("{:?}", params);

    let record;
    match records::InputData::read_from_file("./res/OS_INP.DAT") {
        Ok(t) => record = t,
        Err(e) => panic!("{}", e)
    }
    println!("{:?}", record);

}
