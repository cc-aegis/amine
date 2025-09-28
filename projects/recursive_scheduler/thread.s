; Thread { stack: &any, children: Vec*, alive: bool }
    define STACK_SIZE #128
    include vec.s

OS_THREAD_RS:
    dw #nullptr
CURR_THREAD:
    dw #nullptr

; this WILL override ALL registers
; store all valuable registers yourself before calling
thread::schedule(Thread*):
    push rb
    write OS_THREAD_RS rs
    write CURR_THREAD [#-3]
    read rs [#-3]

    pop rf
    popt rb rd
    popt rr rg
    popt r6 r7
    popt r4 r5
    popt r2 r3
    popt r0 r1
    ret

thread::next():
    pusht r0 r1
    pusht r2 r3
    pusht r4 r5
    pusht r6 r7
    pusht rr rg
    pusht rb rd
    push rf

    read r1 CURR_THREAD
    write r1 rs
    read rs OS_THREAD_RS
    pop rb
    ; TODO: remove dead children
    ; schedule children
    lookup r1 #1
    pusht r1 thread::schedule(Thread*)
    call Vec::for_each_volatile(Vec*,fn(any)*)
    sub rs #2
    ret



; initial thread: b:&b i:&main r0=0 0 0 0 0 0 0 0 rr=0 rg=0 rb=&r0, rd=0 rf=0
thread::spawn(fn()*)->Thread*:
    ; let stack = mem::alloc(STACK_SIZE); (two copies r0, r1)
    ; stack[0] = stack;
    ; stack[1] = main;
    ; stack[12] = stack + 2; (r0 <- r1)
    ; let thread = mem::alloc(3); (r0)
    ; thread[0] = stack + 15;
    ; let children = Vec::new(); (r1)
    ; thread[1] = children;
    ; thread[2] = 1;
    ; thread
    pusht r1 #128
    call mem::alloc(u16)->any* ; stack
    dec rs
    mov r1 r0
    writeitr r1 r1
    writeitr r1 [#-3]
    add r0 #12
    writeitr r0 r1
    add r1 #13
    push #3
    call mem::alloc(u16)->any* ; thread
    dec rs
    writeitr r0 r1
    mov r1 r0
    call Vec::new()->Vec*
    writeitr r1 r0
    write r1 #true
    sub r1 #2
    mov r0 r1
    ; TODO: push new thread to CURR_THREAD.children if it is not null
    ret