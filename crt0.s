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

        .GLOBAL __aeabi_memset
__aeabi_memset:
        movs    r2, r2
        bxle    lr
1:      strb    r1, [r0], #1
        subs    r2, r2, #1
        bgt     1b
        bx      lr

        .GLOBAL __aeabi_memclr4
__aeabi_memclr4:
        movs    r1, r1
        bxle    lr
        mov     r2, #0
1:      str     r2, [r0], #4
        subs    r1, r1, #4
        bgt     1b
        bx      lr

        .GLOBAL __aeabi_memclr
__aeabi_memclr:
        movs    r1, r1
        bxle    lr
        mov     r2, #0
1:      strb    r2, [r0], #1
        subs    r1, r1, #1
        bgt     1b
        bx      lr

        .GLOBAL __aeabi_memcpy4
__aeabi_memcpy4:
1:      subs    r2, r2, #4
        ldrgeb  r3, [r1], #4
        strgeb  r3, [r0], #4
        bgt     1b
        bx      lr

        .GLOBAL __aeabi_memcpy
__aeabi_memcpy:
1:      subs    r2, r2, #1
        ldrgeb  r3, [r1], #1
        strgeb  r3, [r0], #1
        bgt     1b
        bx      lr

        @ floating point operations are not supported
        .GLOBAL __aeabi_ul2f
        .GLOBAL __aeabi_ul2d
__aeabi_ul2f:
__aeabi_ul2d:
1:      b       1b

        @ multi-threading is also not supportd
        .GLOBAL __sync_val_compare_and_swap_1
        .GLOBAL __sync_val_compare_and_swap_2
        .GLOBAL __sync_val_compare_and_swap_4
__sync_val_compare_and_swap_1:
__sync_val_compare_and_swap_2:
__sync_val_compare_and_swap_4:
1:      b       1b

        @ 64 bit integers are not supported
        .GLOBAL __aeabi_uldivmod
__aeabi_uldivmod:
        b       __aeabi_uldivmod

        .GLOBAL __aeabi_uidivmod
        .GLOBAL __aeabi_uidiv
__aeabi_uidivmod:
__aeabi_uidiv:
        cmp     r1, #0
        bxeq    lr
        mov     r3, #1
1:      cmp     r2, r1
        bge     2f
        movs    r2, r1, lsl#1
        movcc   r1, r2
        movcc   r3, r3, lsl#1
        bcc     1b
2:      mov     r2, #0
3:      cmp     r0, r1
        subge   r0, r0, r1
        addge   r2, r2, r3
        movs    r3, r3, lsr#1
        movne   r2, r2, lsr#1
        bne     3b
        mov     r1, r0
        mov     r0, r2
        bx      lr

        .GLOBAL memcmp
memcmp:
        mov     r3, r0
        mov     r0, #0
        movs    r2, r2
        bxeq    lr
1:      ldrb    r0, [r3], #1
        ldrb    r12, [r0], #1
        subs    r0, r0, r12
        bxne    lr
        subs    r2, r2, #1
        bgt     1b
        bx      lr

        .GLOBAL __mulodi4
__mulodi4:
        smull   r0, r3, r1, r0
        movs    r3, r3
        addnes  r3, r3, #1
        movne   r3, #1
        str     r3, [r2]
        bx      lr

        .END
