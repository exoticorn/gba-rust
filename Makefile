ARM_TOOLS ?= $(HOME)/sdks/devkitARM/arm-none-eabi/bin

default: out out/snake.gba

out:
	mkdir -p out

out/%.ll: %.rs
	rustc --emit=ir $*.rs -O -Z no-landing-pads --target arm-none-linux -o out/$*.ll --dep-info out/$*.d

out/%.gba: out/%.ll crt0.s
	llc -O3 -march=arm -mcpu=arm7tdmi out/$*.ll
	$(ARM_TOOLS)/as -o out/$*.o out/$*.s
	$(ARM_TOOLS)/as -o out/crt0.o crt0.s
	$(ARM_TOOLS)/ld -T linker.ld -o out/$*.elf out/crt0.o out/$*.o
	$(ARM_TOOLS)/objcopy -O binary out/$*.elf out/$*.gba

-include out/*.d
