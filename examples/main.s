start:
    mov rs STACK
    call main()
    exit

    define NUM #317
    include add.s

main():
    dbg NUM
.loop:
    jmp .loop

STACK: