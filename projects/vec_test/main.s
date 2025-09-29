start:
    mov rs STACK
    call main()
    exit

dependencies:
    include vec.s

main():
    call Vec::new()->Vec*
    mov r1 r0
    dbg r1

    pusht r1 #1
    dbg #1
    call Vec::push(Vec*,any)
    sub rs #2

    pusht r1 #2
    dbg #2
    call Vec::push(Vec*,any)
    sub rs #2

    pusht r1 #3
    dbg #3
    call Vec::push(Vec*,any)
    sub rs #2

    pusht r1 #4
    dbg #4
    call Vec::push(Vec*,any)
    sub rs #2

    pusht r1 #5
    dbg #5
    call Vec::push(Vec*,any)
    sub rs #2

    ret

STACK: