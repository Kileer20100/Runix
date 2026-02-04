# **Полный переход от `bootloader` к Limine в Runix**

## **1. Почему мы отказываемся от `bootloader`?**

Крейт [`bootloader`](https://github.com/rust-osdev/bootloader) — это удобный инструмент для первых экспериментов. Он:
- Автоматически компилирует загрузочный сектор.
- Переключает CPU в 64-битный режим.
- Настраивает базовую таблицу страниц.
- Инициализирует GDT и стек.
- Передаёт управление функции `_start`.

Это позволяет сразу писать Rust-код и видеть вывод. Но **это же и его главный недостаток**: он принимает за нас ключевые решения, которые **мы должны принимать сами**, если хотим построить настоящую операционную систему.

### **Что скрывает `bootloader`?**
| Компонент | Что делает `bootloader` | Почему это плохо для Runix |
|----------|--------------------------|----------------------------|
| **GDT** | Создаёт фиксированную таблицу дескрипторов | Мы не можем контролировать сегментацию, что критично при переключении задач |
| **Стек** | Выделяет стек по фиксированному адресу | Мы не можем размещать стек там, где нужно (например, в верхней памяти или отдельно на каждом ядре) |
| **Память** | Загружает ядро в заранее выбранный регион | Мы не знаем, где находится ядро, и не можем управлять физической памятью |
| **Прерывания** | Не настраивает IDT | Мы не можем обрабатывать исключения или аппаратные прерывания до инициализации |
| **Информация о системе** | Не передаёт данные от firmware | Мы не получаем карту памяти, ACPI, SMP-информацию |

### **Что даёт Limine?**
Limine — это **загрузчик-протокол**, а не среда выполнения. Он:
- Переключает CPU в 64-битный режим.
- **Не настраивает GDT, IDT, стек, таблицы страниц**.
- **Передаёт всю информацию о системе через стандартизированные структуры**.
- Позволяет нам **самим решать**, как устроена наша система.

Это соответствует философии Runix: **мы не используем готовые решения — мы их создаём**.

---

## **2. Архитектурные требования к ядру под Limine**

Limine требует от ядра следующего:

1. **Точка входа `_start`**  
   Должна быть помечена как `#[no_mangle]`, иметь сигнатуру `extern "C" fn() -> !`.

2. **Секция `.requests`**  
   В ELF-файле должна существовать секция `.requests`, содержащая:
   - Маркер начала (`RequestsStartMarker`)
   - Один или несколько запросов (`FramebufferRequest`, `MemoryMapRequest` и т.д.)
   - Маркер конца (`RequestsEndMarker`)

3. **Высокий адрес загрузки (higher-half kernel)**  
   Ядро должно быть линковано по адресу `0xffffffff80000000` (стандарт для x86_64).

4. **Отсутствие стандартной среды выполнения**  
   Никаких `_init`, `_start_c`, `__libc_start_main` — только чистый `_start`.

---

## **3. Конфигурация Cargo.toml**

```toml
[package]
name = "Runix"
version = "0.1.0"
edition = "2024"

[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"
opt-level = "z"
lto = true
codegen-units = 1

# Указываем кастомную цель
[build]
target = "x86_64-kernel.json"

# Зависимости
[dependencies]
limine = "0.5"                 # Работа с протоколом Limine
x86_64 = "0.14"                # Доступ к регистрам и инструкциям
linked_list_allocator = "0.9"  # Аллокатор для no_std
spin = "0.9"                   # Spinlock для глобального состояния

# Включаем build-std (требуется nightly)
[unstable]
build-std = ["core", "alloc", "compiler_builtins"]
build-std-features = ["compiler-builtins-mem"]

# Настройки линковки
[target.'cfg(target_os = "none")']
rustflags = [
    "-C", "link-arg=-nostartfiles",
    "-C", "link-arg=-Tkernel.ld",
    "-C", "link-arg=-static",
    "-C", "link-arg=-no-pie",
]
```

> **Пояснение флагов**:
> - `-nostartfiles` — отключает CRT (`crt0.o`, `_start_c` и т.д.)
> - `-Tkernel.ld` — использует наш линковочный скрипт
> - `-no-pie` — отключает position-independent code (ядро должно быть по фиксированному адресу)

---

## **4. Целевая спецификация: `x86_64-kernel.json`**

```json
{
"llvm-target": "x86_64-unknown-none",
"data-layout": "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128",
"arch": "x86_64",
"target-endian": "little",
"target-pointer-width": 64,
"target-c-int-width": 32,
"os": "none",
"executables": true,
"linker-flavor": "ld.lld",
"linker": "rust-lld",
"panic-strategy": "abort",
"disable-redzone": true,
"features": "-mmx,-sse,+soft-float",
"rustc-abi": "x86-softfloat"
}
```

> **Ключевые параметры**:
> - `"os": "none"` — bare-metal
> - `"disable-redzone": true` — предотвращает повреждение стека при прерываниях
> - `"features": "-mmx,-sse"` — отключаем SIMD, чтобы не сохранять FPU-состояние

---

## **5. Линковочный скрипт: `kernel.ld`**

```ld
/* Tell the linker that we want an x86_64 ELF64 output file */
OUTPUT_FORMAT(elf64-x86-64)

/* We want the symbol kmain to be our entry point */
ENTRY(kmain)

/* Define the program headers we want so the bootloader gives us the right */
/* MMU permissions; this also allows us to exert more control over the linking */
/* process. */
PHDRS
{
    text    PT_LOAD;
    rodata  PT_LOAD;
    data    PT_LOAD;
}

SECTIONS
{
    /* We want to be placed in the topmost 2GiB of the address space, for optimisations */
    /* and because that is what the Limine spec mandates. */
    /* Any address in this region will do, but often 0xffffffff80000000 is chosen as */
    /* that is the beginning of the region. */
    . = 0xffffffff80000000;

    .text : {
        *(.text .text.*)
    } :text

    /* Move to the next memory page for .rodata */
    . = ALIGN(CONSTANT(MAXPAGESIZE));

    .rodata : {
        *(.rodata .rodata.*)
    } :rodata

    /* Move to the next memory page for .data */
    . = ALIGN(CONSTANT(MAXPAGESIZE));

    .data : {
        *(.data .data.*)

        /* Place the sections that contain the Limine requests as part of the .data */
        /* output section. */
        KEEP(*(.requests_start_marker))
        KEEP(*(.requests))
        KEEP(*(.requests_end_marker))
    } :data

    /* NOTE: .bss needs to be the last thing mapped to :data, otherwise lots of */
    /* unnecessary zeros will be written to the binary. */
    /* If you need, for example, .init_array and .fini_array, those should be placed */
    /* above this. */
    .bss : {
        *(.bss .bss.*)
        *(COMMON)
    } :data

    /* Discard .note.* and .eh_frame* since they may cause issues on some hosts. */
    /DISCARD/ : {
        *(.eh_frame*)
        *(.note .note.*)
    }
}

```

> **Почему `.requests` обязателен?**  
> Limine ищет запросы в этой секции. Если она отсутствует или маркеры не совпадают, загрузчик **не найдёт ваши запросы**, и вы не получите framebuffer, память и т.д.

---

## **6. Точка входа: `src/main.rs`**

```rust
#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;
use linked_list_allocator::LockedHeap;
use spin::Mutex;

// Модули Runix
pub mod cpu;
pub mod drivers;

// Глобальный аллокатор (инициализируется позже)
#[global_allocator]
static ALLOCATOR: LockedHeap<Mutex<linked_list_allocator::LinkedListAllocator>> =
    LockedHeap::new(Mutex::new(linked_list_allocator::LinkedListAllocator::new()));

// === ЗАПРОСЫ LIMINE ===
use limine::BaseRevision;
use limine::request::{FramebufferRequest, RequestsStartMarker, RequestsEndMarker};

// Эти переменные ДОЛЖНЫ быть в секции .requests
#[used]
#[link_section = ".requests"]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[link_section = ".requests"]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
#[link_section = ".requests_start_marker"]
static _START_MARKER: RequestsStartMarker = RequestsStartMarker::new();

#[used]
#[link_section = ".requests_end_marker"]
static _END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

// === ТОЧКА ВХОДА ===
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Отладочный вывод через порт 0xE9 (SeaBIOS)
    unsafe {
        use x86_64::instructions::port::Port;
        let mut port = Port::<u8>::new(0xE9);
        let msg = b"Runix kernel started!\n";
        for &byte in msg {
            port.write(byte);
        }
    }

    // Вызов ассемблерной функции (уже интегрированной)
    cpu::cpu_info();

    // Бесконечный цикл — пока нет планировщика
    loop {
        unsafe { core::arch::asm!("hlt"); }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
```

> **Важно**:
> - Все запросы (`BASE_REVISION`, `FRAMEBUFFER_REQUEST`, маркеры) **обязательны**.
> - `BASE_REVISION` гарантирует совместимость с версией протокола.
> - `FRAMEBUFFER_REQUEST` — единственный способ получить видеобуфер в UEFI/BIOS.

---

## **7. Интеграция ассемблера: `build.rs`**

```rust
use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let asm_file = "src/cpu/cpu_info.asm";
    let obj_file = out_dir.join("cpu_info.o");
    let lib_file = out_dir.join("libasm.a");

    println!("cargo:rerun-if-changed={}", asm_file);

    // Компиляция через NASM
    match Command::new("nasm")
        .args(&["-f", "elf64", asm_file, "-o", obj_file.to_str().unwrap()])
        .status()
    {
        Ok(status) if status.success() => {
            // Создание статической библиотеки
            Command::new("ar")
                .args(&["rcs", lib_file.to_str().unwrap(), obj_file.to_str().unwrap()])
                .status()
                .expect("Не удалось создать libasm.a");

            println!("cargo:rustc-link-search=native={}", out_dir.display());
            println!("cargo:rustc-link-lib=static=asm");
        }
        _ => {
            // Если NASM нет — создаём пустую библиотеку
            Command::new("ar")
                .args(&["rcs", lib_file.to_str().unwrap()])
                .status()
                .ok();
            println!("cargo:warning=NASM не найден. Ассемблерный код не скомпилирован.");
        }
    }
}
```

> Это обеспечивает **полностью автоматическую сборку ASM → Rust**, без ручных шагов.

---

## **8. Сборка ISO: Makefile**

```makefile
KARCH = x86_64
IMAGE_NAME = Runix
QEMUFLAGS = -m 2G -serial mon:stdio -no-reboot -no-shutdown

.PHONY: all run kernel clean

all: $(IMAGE_NAME).iso

run: $(IMAGE_NAME).iso
	qemu-system-$(KARCH) -cdrom $(IMAGE_NAME).iso $(QEMUFLAGS)

kernel:
	cd kernel && cargo build --release
	mkdir -p target
	cp kernel/target/x86_64-kernel/release/Runix target/kernel.elf

limine.conf:
	@echo 'timeout: 3' > limine.conf
	@echo '' >> limine.conf
	@echo '/Runix OS' >> limine.conf
	@echo '    protocol: limine' >> limine.conf
	@echo '    kernel_path: boot():/boot/kernel.elf' >> limine.conf

$(IMAGE_NAME).iso: kernel limine.conf
	rm -rf iso_root
	mkdir -p iso_root/boot/limine iso_root/EFI/BOOT
	cp target/kernel.elf iso_root/boot/
	cp limine.conf iso_root/boot/limine/
	cp limine/limine-bios-cd.bin limine/limine-uefi-cd.bin iso_root/boot/limine/
	cp limine/BOOTX64.EFI iso_root/EFI/BOOT/

	xorriso -as mkisofs \
		-b boot/limine/limine-bios-cd.bin \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		--efi-boot boot/limine/limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		iso_root -o $(IMAGE_NAME).iso

	./limine/limine bios-install $(IMAGE_NAME).iso

clean:
	cd kernel && cargo clean
	rm -rf iso_root $(IMAGE_NAME).iso target limine.conf
```

> **Структура ISO**:
> ```
> iso_root/
> ├── boot/
> │   ├── kernel.elf
> │   └── limine/
> │       ├── limine-bios-cd.bin
> │       ├── limine-uefi-cd.bin
> │       └── limine.conf
> └── EFI/
>     └── BOOT/
>         └── BOOTX64.EFI
> ```

---

## **9. Запуск в QEMU**

```bash
qemu-system-x86_64 \
    -cdrom Runix.iso \
    -chardev stdio,id=seabios \
    -device isa-debugcon,iobase=0xe9,chardev=seabios \
    -m 2G -serial mon:stdio -no-reboot -no-shutdown
```

- Порт `0xE9` перенаправляется в терминал — вы увидите `"Runix kernel started!"`.
- Если VGA работает — вы увидите `"Add: 3"` от `cpu_info()`.

---

## **10. Что мы получили**

1. **Полный контроль над инициализацией**  
   Теперь мы сами настраиваем GDT, IDT, стек, таблицы страниц.

2. **Доступ к данным от firmware**  
   Через `FRAMEBUFFER_REQUEST.get()` мы получаем:
   - Адрес видеобуфера
   - Ширину/высоту экрана
   - Байт на пиксель
   - Шаг строки (stride)

3. **Подготовка к многозадачности**  
   Поскольку стек не инициализирован, мы можем разместить его где угодно — например, в верхней памяти или отдельно для каждого ядра.

4. **Чистая архитектура**  
   Нет зависимостей от внешних библиотек, кроме `limine` (который — лишь интерфейс). Всё остальное — наш код.

---

## **11. Следующие шаги**

Теперь, когда у нас есть:
- работающий загрузчик (Limine),
- точка входа (`_start`),
- ассемблерная интеграция,
- VGA-драйвер,

мы можем приступить к **фундаментальным механизмам ядра**:

1. **Инициализация GDT**  
   Установка нулевых дескрипторов, подготовка flat memory model.

2. **Настройка IDT и обработка исключений**  
   Page fault, general protection fault, double fault.

3. **Управление физической памятью**  
   Парсинг карты памяти через `MemoryMapRequest`, реализация bitmap-аллокатора.

4. **Переход к виртуальной памяти**  
   Построение 4-уровневой страничной таблицы, включение бита `PG` в `CR0`.

Каждый из этих шагов будет невозможен без полного контроля, который даёт Limine.

---

## **Заключение**

Переход на Limine — это не просто замена одного загрузчика другим. Это **переход от прототипа к настоящей операционной системе**. Мы отказываемся от удобства ради контроля, от абстракции ради понимания.

Этот шаг знаменует окончание «фазы Hello World» и начало **реальной системной инженерии**. Всё, что будет дальше — строится на этом фундаменте.

Runix теперь — не просто программа, которая что-то выводит. Это **ядро**, которое знает, как устроен компьютер, и готово взять на себя ответственность за его управление.

[]()