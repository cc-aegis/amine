    include thread.s

test::main():
    mov r0 #0
.loop:
    dbg r0
    inc r0
    call thread::next()
    jmp .loop