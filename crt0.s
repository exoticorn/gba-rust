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

        .GLOBAL memset
memset:
        movs    r2, r2
        bxle    lr
_loop:  strb    r1, [r0], #1
        subs    r2, r2, #1
        bgt     _loop
        bx      lr

        .GLOBAL __aeabi_memclr4
__aeabi_memclr4:
        movs    r1, r1
        bxle    lr
        mov     r2, #0
_loop2: str     r2, [r0], #4
        subs    r1, r1, #1
        bgt     _loop2
        bx      lr

        .END
