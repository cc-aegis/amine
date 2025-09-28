start:
    mov rs STACK
    call main()
    exit

dependencies:
    include mem.s
    include vec.s
    include thread.s
    include apps/test.s
    include apps/child_test.s

main():
    ; setup
    push child_test::main()
    call thread::spawn(fn()*)->Thread*
    dec rs
.loop:
    pusht r0 r0
    call thread::schedule(Thread*)
    dec rs
    pop r0
    ; TODO: render
    jmp .loop

STACK: