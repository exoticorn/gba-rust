BINUTILS_PREFIX=arm-none-eabi-
RUST_LIBS=libs

default: out out/snake.gba

debug: out out/snake-debug.gba

out:
	mkdir -p out

cargo-build-release:
	rustup run nightly `which xargo` build --release --target=gba

cargo-build-debug:
	rustup run nightly `which xargo` build --target=gba

out/snake.gba: cargo-build-release crt0.s
	$(BINUTILS_PREFIX)as -o out/crt0.o crt0.s
	$(BINUTILS_PREFIX)ld -T linker.ld -o out/snake.elf out/crt0.o target/gba/release/libgba_snake.a
	$(BINUTILS_PREFIX)objcopy -O binary out/snake.elf out/snake.gba

out/snake-debug.gba: cargo-build-debug crt0.s
	$(BINUTILS_PREFIX)as -o out/crt0.o crt0.s
	$(BINUTILS_PREFIX)ld -T linker.ld -o out/snake-debug.elf out/crt0.o target/gba/debug/libgba_snake.a
	$(BINUTILS_PREFIX)objcopy -O binary out/snake-debug.elf out/snake-debug.gba
