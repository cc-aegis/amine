random::SEED:
    dw #44257

random::next()->any:
    read r0 random::SEED
    push r0
    and [#0] #1
    sshr r0
    jrz [#0] .end
    xor r0 #46080
.end:
    dec rs
    write random::SEED r0
    ret

random::seed(any):
    write random::SEED [#-3]
    ret

random::salt(any):
    read r0 random::SEED
    xor r0 [#-3]
    write random::SEED r0
    ret