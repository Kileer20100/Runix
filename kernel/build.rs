use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    
    // 1. Компилируем ассемблер
    let asm_file = "src/cpu/cpu_info.asm";
    let obj_file = out_dir.join("cpu_info.o");
    let lib_file = out_dir.join("libasm.a");
    
    println!("cargo:rerun-if-changed={}", asm_file);
    
    // Проверяем что nasm установлен
    let nasm_status = Command::new("nasm")
        .args(&["-f", "elf64", asm_file, "-o", obj_file.to_str().unwrap()])
        .status();
    
    match nasm_status {
        Ok(status) if status.success() => {
            // 2. Создаем библиотеку
            Command::new("ar")
                .args(&["rcs", lib_file.to_str().unwrap(), obj_file.to_str().unwrap()])
                .status()
                .expect("Ошибка создания библиотеки");
            
            println!("cargo:rustc-link-search=native={}", out_dir.display());
            println!("cargo:rustc-link-lib=static=asm");
        }
        _ => {
            println!("cargo:warning=nasm не найден, пропускаем ассемблер");
            // Создаем пустую библиотеку чтобы не было ошибки линковки
            Command::new("ar")
                .args(&["rcs", lib_file.to_str().unwrap()])
                .status()
                .ok();
        }
    }
}