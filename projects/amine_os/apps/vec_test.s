    include vec.s

VecTest:
    call Vec::new()->Vec*
    mov r1 r0

    pusht r1 #1
    call Vec::push(Vec*,any)
    sub rs #2

    pusht r1 #2
    call Vec::push(Vec*,any)
    sub rs #2

    pusht r1 #3
    call Vec::push(Vec*,any)
    sub rs #2

    pusht r1 #4
    call Vec::push(Vec*,any)
    sub rs #2

    pusht r1 #5
    call Vec::push(Vec*,any)
    sub rs #2

    pusht r1 #6
    call Vec::push(Vec*,any)
    sub rs #2

    read r1 r1
    readitr r0 r1
    dbg r0
    readitr r0 r1
    dbg r0
    readitr r0 r1
    dbg r0
    readitr r0 r1
    dbg r0
    readitr r0 r1
    dbg r0
    readitr r0 r1
    dbg r0

    exit