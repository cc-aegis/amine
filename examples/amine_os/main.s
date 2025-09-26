start:
    mov rs #8000
    push Fibonacci
    call thread::spawn(App*)->Thread*
    dec rs
    add r0 #15
    write THREAD_2 r0
    write THREAD_2b r0
    ; call VecTest
    call Mode7::main()
    exit

dependencies:
    include mem.s
    include apps/mode7.s
    include apps/vec_test.s




thread::next():
    pusht r0 r1
    pusht r2 r3
    pusht r4 r5
    pusht r6 r7
    pusht rr rg
    pusht rb rd
    push rf
    read r0 CURR_THREAD
    write r0 rs
    xor r0 #1
    write CURR_THREAD r0
    read rs r0
    pop rf
    popt rb rd
    popt rr rg
    popt r6 r7
    popt r4 r5
    popt r2 r3
    popt r0 r1
    ret

; TODO: attach to children list
; TODO: return Thread* instead of just stack itself
thread::spawn(App*)->Thread*:
    pusht r1 #128
    call mem::alloc(u16)->any* ; stack
    dec rs
    mov r1 r0
    writeitr r1 r1
    writeitr r1 [#-3]
    add r0 #12
    write r0 r1
    sub r0 #12
    pop r1
    ret



; TODO: loop recursively through thread tree
CURR_THREAD:
    dw THREAD_1

THREAD_2b:
    dw #nullptr
; rs
THREAD_1:
    dw #8000
THREAD_2:
    dw #nullptr


; struct Thread { stack: &any, ctx: &u16, children: Vec<&Thread> }

Fibonacci:
    mov r0 #0
    mov r1 #1
Fibonacci.loop:
    add r0 r1
    dbg r0
    add r1 r0
    dbg r1
    call thread::next()
    jmp Fibonacci.loop


Counter:
    mov r0 #0
Counter.loop:
    dbg r0
    inc r0
    call thread::next()
    jmp Counter.loop

; TODO: create launch state for Counter
