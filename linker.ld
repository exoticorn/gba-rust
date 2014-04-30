OUTPUT_FORMAT("elf32-littlearm", "elf32-littlearm", "elf32-littlearm")
OUTPUT_ARCH(arm)
/*ENTRY(_start)*/

SECTIONS {
  . = 0x8000000;

  .text : {
    *(.text .rodata)
  }

  .data : { *(.data) }
  .bss : { *(.bss) }
}
