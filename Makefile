ARM_TOOLS ?= $(HOME)/sdks/devkitARM/arm-none-eabi/bin
RUST_LIBS=libs

default: out out/snake.gba

out:
	mkdir -p out

cargo-build:
	rustup run nightly `which xargo` build --release --target=gba

out/%.gba: cargo-build crt0.s
	$(ARM_TOOLS)/as -o out/crt0.o crt0.s
	$(ARM_TOOLS)/ld -T linker.ld -o out/$*.elf out/crt0.o target/gba/release/libgba_snake.a
	$(ARM_TOOLS)/objcopy -O binary out/$*.elf out/$*.gba
