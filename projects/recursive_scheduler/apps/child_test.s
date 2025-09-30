    include apps/test.s

child_test::main():
.loop:
    dbg #12345

    push test::main()
    call thread::spawn(fn()*)->Thread* ; TODO: make ::spawn auto push to .children
    dec rs
    call thread::next()
    jmp .loop