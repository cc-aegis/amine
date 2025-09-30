    include thread.s
    include random.s

random_replicate::main():
.loop:
    call random::next()->any
    and r0 #511
    jrnz r0 .skip
.inner_loop:
    call random::next()->any
    and r0 #1
    jrnz r0 .skip

    push random_replicate::main()
    call thread::spawn(fn()*)->Thread*
    dec rs

    jmp .inner_loop
.skip:
    call thread::next()
    jmp .loop