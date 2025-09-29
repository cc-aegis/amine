    include apps/test.s

child_test::main():
.loop:
    dbg #12345

    push test::main()
    call thread::spawn(fn()*)->Thread* ; TODO: make ::spawn auto push to .children
    dec rs
    ; fallback:
    read r1 CURR_THREAD
    lookup r1 #1
    pusht r1 r0
    call Vec::push(Vec*,any)
    sub rs #2

    call thread::next()
    jmp .loop