; The address of Option<Coordinate> is passed in the rdi register.
; The representation of Option<Coordinate> is shown in the table above.

.LCPI0_0:
        .quad   0x8000000000000000              ; Constants used for flipping the signed bit..
        .quad   0x8000000000000000              ; ..of a 64-bit floating point number.
example::make_quad_coordinates:
        sub     rsp, 24                         ; Reserve space for local variables.
        cmp     qword ptr [rdi], 0              ; Check if the discriminator is set to 0 (None case)
        je      .LBB0_1                         ; If the discriminator is 0, jump to the exit point.

        ; == Processing Some case of Option ==
        movups  xmm0, xmmword ptr [rdi + 8]     ; Get the 0ᵗʰ and 1ˢᵗ entry from the Coordinate tuple via a vectorized load
                                                ; (Note the 8-byte offset is needed to skip the discriminator)
        movaps  xmmword ptr [rsp], xmm0         ; Save it into the x and y local variable on the stack.

        ; Requesting memory allocation for Option<Box<[Coordinate; 4]>>.
        ; The array has 4 entries, and each entry needs 16 bytes (for the two f64s)
        ; This adds up to a total of 64 bytes.
        ; Note that no space is needed for the Option discriminator as all 0s (NULL)
        ; can be used to represent the None condition. Any nonzero heap address signifies
        ; the Some condition.

        mov     edi, 64                                 ; Request 64 byte memory
        mov     esi, 8                                  ; Alignment is set to 8 bytes
        call    qword ptr [rip + __rust_alloc@GOTPCREL] ; Request memory allocation. The result is returned in rax.
        test    rax, rax                                ; Check if the memory allocation returned an all zero address (NULL)
        je      .LBB0_5                                 ; If yes, jump to the out of memory error handling
        movaps  xmm1, xmmword ptr [rsp]                 ; Load the x and y local variable into xmm1 (vectorized load)
        movups  xmmword ptr [rax], xmm1                 ; Copy x and y to the heap
        movaps  xmm0, xmmword ptr [rip + .LCPI0_0]      ; Load the constant with the most significant bit set
        xorps   xmm0, xmm1                              ; Flip the sign bit of x and y to store -x and -y in xmm0
        movups  xmmword ptr [rax + 16], xmm0            ; Copy -x and -y to the heap (vectorized store)
        movlps  qword ptr [rax + 32], xmm0              ; Copy -x to the heap
        shufps  xmm1, xmm1, 78                          ; Swap the x and y values in xmm1
        movups  xmmword ptr [rax + 40], xmm1            ; Copy y and x to the heap
        pshufd  xmm0, xmm0, 238                         ; Bring the -y value to the lower 64 bits of xmm0
        movq    qword ptr [rax + 56], xmm0              ; Copy -y to the heap
        add     rsp, 24                                 ; Pop off the local variables from the stack
        ret                                             ; Return the result in rax

        ; == Processing None case of Option ==
        ; Rust implements optimizes Option<Box>, the None case is signaled by an all zero (NULL)
        ; returned to the caller. There is no discriminator needed.

.LBB0_1:
        xor     eax, eax    ; Setting the lower 32 bits of rax to 0
                            ; Upper bits must be 0 to return an all zero rax
        add     rsp, 24     ; Pop off the local variables
        ret                 ; Return NULL to signal None

        ; == Exception processing (memory allocation failed)
.LBB0_5:
        mov     edi, 64         ; Size of the failed allocation (64-bit)
        mov     esi, 8          ; Alignment of the failed allocation (8-byte)
        call    qword ptr [rip + alloc::alloc::handle_alloc_error@GOTPCREL] ; Call alloc error handler
        ud2                     ; Raise invalid opcode exception to trigger the exception handler.