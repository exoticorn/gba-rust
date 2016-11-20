ARM_TOOLS ?= $(HOME)/sdks/devkitARM/arm-none-eabi/bin

default: out out/snake.gba

out:
	mkdir -p out

cargo-build:
	cargo build --release --target=arm-unknown-linux-gnueabi

out/%.gba: cargo-build crt0.s
	$(ARM_TOOLS)/as -o out/crt0.o crt0.s
	$(ARM_TOOLS)/ld -T linker.ld -o out/$*.elf out/crt0.o target/arm-unknown-linux-gnueabi/release/libgba_snake.a
	$(ARM_TOOLS)/objcopy -O binary out/$*.elf out/$*.gba
