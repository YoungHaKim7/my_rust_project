# Assembly Files: Difference between .a .s .asm 

https://stackoverflow.com/questions/34098596/assembly-files-difference-between-a-s-asm

.s 컴파일해서 나오는 자동 어셈플리 파일

<br>

.asm 손으로 쓴 어셈플리 파일

<br>

 Unix/Linux systems:

- .a

.a is the usual extension for static libraries (aka Archives of multiple .o files, made with ar(1)). Dynamic libraries, aka Shared Objects, use .so.

- .s

.s is used for asm compiler output. (gcc -S foo.c produces asm output, with a default filename of foo.s)

- .S (.asm)

.S is used for hand-written asm source files.

<br>

<hr>


# Rust to assembly: Arrays, Tuples, Box, and Option handling

https://www.eventhelix.com/rust/rust-to-assembly-arrays-option-box/

<br>


<hr>

# Intel __ Introduction_to_x64 assembly

https://www.intel.com/content/dam/develop/external/us/en/documents/introduction-to-x64-assembly-181178.pdf
