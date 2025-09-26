# TODOs
- replace String with Rc<str> and compare
- recursively call link on Vec\[Instruction\] before preparsing/compiling in case you intent to use includes
- experimentally rewrite emulator in Zig
- rewrite code so that registers can no longer appear as OpValue (disallow in define, dw through type system)