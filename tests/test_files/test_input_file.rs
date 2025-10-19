pub static TEST_FILE_LOG: &str = "\
the begining\n
#< test_sequence <#
#= Cmd52 write: bool(true) ln: u8(1) add: x32(0x01043) data: x8(0x80) =#
#= Cmd53 write: bool(true) ln: u8(1) add: x32(0x01043) inc: bool(true)
data: x8([0x80,0x05]) =#
#= Cmd52 write: bool(true) ln: u8(1) add: x32(0x01043) data: x8(0x80) =#
just trash
#= Cmd53 write: bool(false) ln: u8(0) add: x32(0x01043) inc: bool(true)
data: x8([0x80,0x05,
0x20,0xfe,
0xc
4,
0x31,0x4,0x60,0xce]) =#
#[ ReadEfuse [#
#- ReadEfuse offset: x16(0x5ea) efuse_start: x8(0x0) size: u8(1) read_efuse_cnt: u32(10000) efuse_ctrl: x8(0x30) dv_sel: id(EfuseAccess::DDV) -#
#= Cmd52 write: bool(true) ln: u8(1) add: x32(0x01043) data: x8(0x80) =#
#- ReadEfuse map_ptr: x64(00000000da5708c1) -#
I am not a record
e neither
#] ReadEfuse ]#
#= AnotherCmd num: f32(3.565) adv: id(EfuseAccess::DAV) top: i32(-2500) adu: id(EfuseAccess::DXV) dot: i8([-25,-69,2]) =#
#> test_sequence >#
#= UnexistingCmd write: bool(true) ln: u8(1) add: x32(0x01043) data: x8(0x80) =#\n";
