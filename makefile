all: bootimage boot
	
bootimage:
	cargo bootimage 

build:
	cargo build --target x86_64-panic_os.json

boot:
	qemu-system-x86_64 -drive format=raw,file=target/x86_64-panic_os/debug/bootimage-panic_os.bin

