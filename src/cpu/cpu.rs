use crate::println;


unsafe extern "C"{
fn add(a: i64, b: i64) -> i64;
}

pub fn cpu_info(){


    println!("Core: {}", unsafe {
        add(1,2)
    });

}