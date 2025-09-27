    include main.s

FillRed::main():
    mov rd #1
    ctx #3 #192
.loop0:
    ctx #0 #1
    mov r0 #128
.loop1:
    ctx #1 r0
    mov r1 #96
.loop2:
    ctx #2 r1
    send
    jrnzdec r1 .loop2
    jrnzdec r0 .loop1
    ctx #0 #2
    send
    call thread::next()
    jmp .loop0