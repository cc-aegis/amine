; Vec { arr: any*, len: u16, cap: u16 }

    include mem.s

Vec::new()->Vec*:
    push #4
    call mem::alloc(u16)->any*
    pusht r0 r3
    call mem::alloc(u16)->any*
    writeitr r0 [#0]
    writeitr r0 #0
    write r0 #4
    sub r0 #2
    ret


Vec::push(Vec*,any):
    ; cap:r0 len:r1 arr:r2
    pusht r1 r2
    ; if vec.len == vec.cap {
    mov r0 [#-4]
    readitr r2 r0
    readitr r1 r0
    read r0 r0
    sub r0 r1
    jrnz r0 Vec::push(&Vec,any).skip
    pusht r3 r4
    ;     let cap = vec.cap << 1;
    mov r3 r0
    shl r3 #1
    ;     let arr = mem::alloc(cap);
    push r3
    call mem::alloc(u16)->any*
    mov r4 r0
    ;     mem::copy(arr, vec.arr, vec.cap);
    pusht r4 r2
    mov r0 [#-4]
    lookup r0 #2
    push r0
    call mem::copy(any*,any*,u16)
    ;     mem::free(vec.arr);
    read r0 [#-4]
    push r0
    call mem::free(any*)
    sub rs #5
    ;     vec.arr = arr;
    mov r0 [#-4]
    write r0 r4
    ;     vec.cap = cap;
    add r0 #2
    write r2 r3
    ; }
    popt r3 r4
Vec::push(&Vec,any).skip:
    ; vec.arr[vec.len] = val;
    add r2 r1
    write r2 [#-3]
    ; vec.len += 1;
    inc r1
    inc [#-4]
    write [#-4] r1
    popt r1 r2
    ret

Vec::for_each(Vec*,fn(any)*):
    pusht r1 r2
    readitr r1 [#-4] ; arr
    read r2 [#-4] ; len
    jmp .cond
.loop:
    readitr r0 r1 ; element
    push r0
    call [#-3]
    dec rs
.cond:
    jrnzdec r1 .loop
    popt r1 r2
    ret

; may be used with functions that disrupt register state
Vec::for_each_volatile(Vec*,fn(any)*):
    pusht r1 r2
    readitr r0 [#-4] ; arr
    read r1 [#-4] ; len
    jmp .cond
.loop:
    readitr r2 r0 ; element
    pusht r0 r1
    push r2
    call [#-3]
    dec rs
    popt r0 r1
.cond:
    jrnzdec r1 .loop
    popt r1 r2
    ret