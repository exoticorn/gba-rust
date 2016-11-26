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

  __exidx_start = .;
  /* Exception handling, exidx needs a dedicated section */
  .ARM.exidx : ALIGN(4)
  {
  *(.ARM.exidx* .gnu.linkonce.armexidx.*)
  }
  __exidx_end = .;
}
