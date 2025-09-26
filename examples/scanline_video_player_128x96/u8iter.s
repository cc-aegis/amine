; struct U8Iter { ptr: u8*, curr: u16 }
; curr will NEVER be 0 unless it is explicitly set as nullptr

    include mem.s

U8Iter::new(u16*)->U8Iter*:
    push #2
    call mem::alloc(u16)->any*
    writeitr r0 [#-3]
    write r0 #nullptr
    dec r0
    ret

U8Iter::next(U8Iter*)->u8:
    push r1
    readitr r1 [#-3]
    read r0 [#-3]
    jrz r0 .lhs
.rhs:
    and r0 #255
    write [#-3] #nullptr
    pop r1
    ret
.lhs:
    readitr r0 r1
    dec [#-3]
    writeitr [#-3] r1
    write [#-3] r0
    shr r0 #8
    pop r1
    ret
