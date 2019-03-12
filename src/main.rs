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

    let all_records;
    match records::InputData::read_from_file("./res/OS_INP.DAT", 10) {
        Ok(t) => all_records = t,
        Err(e) => panic!("{}", e)
    }
    for r in all_records {
        println!("{:?}", r);
    }

}
