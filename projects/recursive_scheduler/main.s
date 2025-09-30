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
    include apps/self_replicate.s
    include apps/random_replicate.s

main():
    ; setup
    push random_replicate::main() ; child_test::main() ; self_replicate::main()
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