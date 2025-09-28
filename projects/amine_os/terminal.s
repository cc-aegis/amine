; Terminal { len: u16, off: u16, buf: &char[512], input_buf: &char[16], input_idx: u8|{-1} }

Terminal::new()->Terminal*:
    callw mem::alloc(u16)->any* #5
    store r1 r0
    writeitr r1 #0
    writeitr r1 #0
    callw mem::alloc(u16)


Terminal::free(Terminal*):

Terminal::push(Terminal*,char*):

Terminal::readln(Terminal*)->char*:

Terminal::render(Terminal*):
