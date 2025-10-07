pub static TEST_FILE_LOG: &str = "\
the begining\n\
[   62.996337] #: test_sequence
[   62.996339] #= Cmd52 write: bool(true) fn: u8(1) add: x32(0x01043) data: x8(0x80) =#
[   64.049750] #= Cmd53 write: bool(true) fn: u8(1) add: x32(0x01043) inc: bool(true)
[   64.054214] data: x8([0x80,0x05]) =#
[   62.996339] #= Cmd52 write: bool(true) fn: u8(1) add: x32(0x01043) data: x8(0x80) =#
Just trash
[   64.049750] #= Cmd53 write: bool(false) fn: u8(0) add: x32(0x01043) inc: bool(true)
[   64.054214] data: x8([0x80,0x05,
[   64.054214] 0x20,0xfe,0xc4,
[   64.054214] 0x31,0x4,0x60,0xce]) =#
[   58.432667] #[ Read_Efuse
[   58.437030] #- offset: x16(0x5ea) efuse_start: x8(0x0) size: u8(1) read_efuse_cnt: u32(10000) efuse_ctrl: x8(0x30) dv_sel: id(EfuseAccess::DDV) -#\n
[   62.996339] #= Cmd52 write: bool(true) fn: u8(1) add: x32(0x01043) data: x8(0x80) =#
[   58.439921] #- map_ptr: x64(00000000da5708c1) -#
[   64.592339] I am not a record
Me neither
[   64.921023] Read_Efuse #]
[   62.996339] #= Another_Cmd num: f32(3.565) adv: id(EfuseAccess::DAV) top: i32(-2500) adu: id(EfuseAccess::DXV) dot: i8([-25,-69,2]) =#
[   64.921023] test_sequence :#
[   62.996339] #= Unexisting_Cmd write: bool(true) fn: u8(1) add: x32(0x01043) data: x8(0x80) =#\n";
