start:
    mov rs STACK
    call main()
    exit

main():
    mov r0 #0
    mov r1 #1
.loop:
    add r0 r1
    dbg r0
    add r1 r0
    dbg r1
    jmp .loop

STACK: