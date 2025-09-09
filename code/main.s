start:
    mov rs STACK
    call main()
    exit

    define NUM #317

main():
    dbg NUM
.loop:
    jmp .loop

STACK: