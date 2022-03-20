
arch = riscv32i-unknown-none-elf

build_bin:
	cargo build --release
	rust-objcopy target/${arch}/release/lemu_test --strip-all -O binary target/${arch}/release/lemu_test.bin

bin_disasm:
	rust-objdump target/${arch}/release/lemu_test -d --arch-name=riscv64

bin_info:
	riscv64-unknown-elf-objdump -h target/${arch}/release/lemu_test

debug_build_bin:
	cargo build
	rust-objcopy target/${arch}/debug/lemu_test --strip-all -O binary target/${arch}/debug/lemu_test.bin