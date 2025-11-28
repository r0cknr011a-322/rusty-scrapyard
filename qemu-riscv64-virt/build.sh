cargo build --release
cp target/riscv64gc-unknown-none-elf/release/qemu-riscv64-virt target/main.elf
llvm-objdump-14 -D target/main.elf > target/asm.dump
