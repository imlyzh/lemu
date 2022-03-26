
arch = rv64im-unknown-none-elf

objdump = riscv64-linux-gnu-objdump

# objdump = riscv64-unknown-elf-objdump

build:
	cargo build --release --target rv64im-unknown-none-elf.json -Zbuild-std=core,panic_abort
	rust-objcopy target/${arch}/release/lemu_test --strip-all -O binary target/${arch}/release/lemu_test.bin

bin_disasm:
	rust-objdump target/${arch}/release/lemu_test -d --arch-name=riscv64

bin_info:
	rust-objdump -h --arch-name=riscv64 target/${arch}/release/lemu_test

debug_build:
	cargo build --target rv64im-unknown-none-elf.json -Zbuild-std=core,panic_abort
	rust-objcopy target/${arch}/debug/lemu_test --strip-all -O binary target/${arch}/debug/lemu_test.bin