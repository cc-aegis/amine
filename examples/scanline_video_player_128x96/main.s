start:
    mov rs STACK
    call main()
    exit

DEVICE_DISPLAY:
    dw #0

VIDEO:
    include video.s
    dw #115 #116 #97 #114 #116 #32 #111 #102 #32 #112 #114 #111 #103 #114 #97 #109
    include u8iter.s

render_frame(U8Iter*):
    pusht r1 r2
    pusht r3 r4
    read rd DEVICE_DISPLAY
    ctx #0 #1
    mov r1 #0 ; x
    mov r2 #0 ; y
    mov r3 #0 ; color
    mov r4 #0 ; r0 clone
.take:
    push [#-3]
    call U8Iter::next(U8Iter*)->u8
    dec rs
    mov r4 r0
    jmp .set_cond
.set:
    ctx #1 r1
    ctx #2 r2
    ctx #3 r3
    send
    sub r2 #95
    jrz r2 .set_next_column
    add r2 #96
.set_cond:
    jrnzdec r0 .set
    sub r4 #255
    jrz r4 .same_color
    mov r4 #255
    sub r4 r3
    mov r3 r4
.same_color:
    jmp .take
.set_next_column:
    mov r2 #0
    sub r1 #127
    jrz r1 .end
    add r1 #128
    jmp .set_cond
.end:
    popt r3 r4
    popt r1 r2
    ret

main():
    ; r1 := U8Iter::new(VIDEO);
    push VIDEO
    call U8Iter::new(u16*)->U8Iter*
    dec rs
    mov r1 r0

.loop:
    push r1
    call render_frame(U8Iter*)
    dec rs
    ctx #0 #2
    send
    read r0 r1
    dbg r0
    ; call wait()
    jmp .loop

    ret

wait():
    push r1
    mov r0 #100
.loop1:
    mov r1 #65535
.loop2:
    jrnzdec r1 .loop2
    jrnzdec r0 .loop1
    pop r1
    ret

STACK: