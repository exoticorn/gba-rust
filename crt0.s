        .TEXT
        .GLOBAL _start
_start:
        .ALIGN
        .CODE 32
        b       _header_end
        .fill   188, 1, 0
_header_end:
        mov     r0, #0
        ldr     lr, =_header_end
        ldr     r5, =main
        bx      r5

        .END
