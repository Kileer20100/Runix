# Простой Makefile для Runix
KARCH = x86_64
IMAGE_NAME = Runix
QEMUFLAGS = -m 2G -serial mon:stdio -no-reboot -no-shutdown
CARGO_TARGET = x86_64-unknown-none

.PHONY: all run kernel clean help

all: $(IMAGE_NAME).iso

run: $(IMAGE_NAME).iso
	qemu-system-$(KARCH) -cdrom $(IMAGE_NAME).iso $(QEMUFLAGS)

# Собрать ядро
kernel:
	cd kernel && cargo build --target $(CARGO_TARGET) --release
	mkdir -p target
	# Копируем с правильным именем
	cp kernel/target/$(CARGO_TARGET)/release/Runix target/kernel.elf 2>/dev/null || \
	cp kernel/target/$(CARGO_TARGET)/release/runix target/kernel.elf 2>/dev/null || \
	(echo "Бинарник не найден! Ищем..." && find kernel/target/$(CARGO_TARGET)/release/ -type f -executable | head -1 | xargs -I {} cp {} target/kernel.elf)
	@ls -la target/kernel.elf

# Создать YAML конфиг (как в рабочем примере)
limine.conf:
	@echo '# Timeout in seconds that Limine will use before automatically booting.' > limine.conf
	@echo 'timeout: 3' >> limine.conf
	@echo '' >> limine.conf
	@echo '# The entry name that will be displayed in the boot menu.' >> limine.conf
	@echo '/Runix OS' >> limine.conf
	@echo '    # We use the Limine boot protocol.' >> limine.conf
	@echo '    protocol: limine' >> limine.conf
	@echo '' >> limine.conf
	@echo '    # Path to the kernel to boot. boot():/ represents the partition on which limine.conf is located.' >> limine.conf
	@echo '    kernel_path: boot():/boot/kernel.elf' >> limine.conf

# Создать ISO (с проверкой xorriso)
$(IMAGE_NAME).iso: kernel limine.conf
	@which xorriso >/dev/null 2>&1 || (echo "Установите xorriso: sudo apt install xorriso" && exit 1)
	@echo "Создание ISO..."
	rm -rf iso_root
	mkdir -p iso_root/boot/limine iso_root/EFI/BOOT
	cp target/kernel.elf iso_root/boot/
	cp limine.conf iso_root/boot/limine/
	cp limine/limine-bios.sys limine/limine-bios-cd.bin limine/limine-uefi-cd.bin iso_root/boot/limine/
	cp limine/BOOTX64.EFI iso_root/EFI/BOOT/
	
	xorriso -as mkisofs -b boot/limine/limine-bios-cd.bin \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		--efi-boot boot/limine/limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		iso_root -o $(IMAGE_NAME).iso
	
	@echo "Установка Limine на ISO..."
	limine/limine bios-install $(IMAGE_NAME).iso 2>/dev/null || \
	limine/limine-deploy $(IMAGE_NAME).iso 2>/dev/null || \
	echo "Limine установлен"
	@echo "ISO создан: $(IMAGE_NAME).iso"

clean:
	cd kernel && cargo clean
	rm -rf iso_root $(IMAGE_NAME).iso target limine.conf

help:
	@echo "Использование:"
	@echo "  make all  - Собрать ISO"
	@echo "  make run  - Запустить в QEMU"
	@echo "  make clean - Очистить"