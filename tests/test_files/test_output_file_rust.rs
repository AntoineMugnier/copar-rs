pub static TEST_FILE_RUST_CONTENT: &str = "\
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationId {
    Cmd52,
    Cmd53,
    ReadEfuse,
    AnotherCmd,
}

#[derive(Debug, Clone)]
pub enum OperationVariant {
    Cmd52(&'static Cmd52),
    Cmd53(&'static Cmd53),
    ReadEfuse(&'static ReadEfuse),
    AnotherCmd(&'static AnotherCmd),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EfuseAccess {
    Ddv,
    Dav,
    Dxv,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cmd52 {
    pub write: bool,
    pub ln: u8,
    pub add: u32,
    pub data: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cmd53 {
    pub write: bool,
    pub ln: u8,
    pub add: u32,
    pub inc: bool,
    pub data: &'static [u8],
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReadEfuse {
    pub offset: u16,
    pub efuse_start: u8,
    pub size: u8,
    pub read_efuse_cnt: u32,
    pub efuse_ctrl: u8,
    pub dv_sel: EfuseAccess,
    pub map_ptr: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AnotherCmd {
    pub num: f32,
    pub adv: EfuseAccess,
    pub top: i32,
    pub adu: EfuseAccess,
    pub dot: &'static [i8],
}

pub static ARRAY_0: & [u8] = &[0x80, 0x5];
pub static ARRAY_1: & [u8] = &[0x80, 0x5, 0x20, 0xfe, 0xc4, 0x31, 0x4, 0x60, 0xce];
pub static ARRAY_2: & [i8] = &[-25, -69, 2];

pub static OPERATION_0: Cmd52 = Cmd52 { write: true, ln: 1, add: 0x1043, data: 0x80 };
pub static OPERATION_1: Cmd53 = Cmd53 { write: true, ln: 1, add: 0x1043, inc: true, data: ARRAY_0 };
pub static OPERATION_2: Cmd53 = Cmd53 { write: false, ln: 0, add: 0x1043, inc: true, data: ARRAY_1 };
pub static OPERATION_3: ReadEfuse = ReadEfuse { offset: 0x5ea, efuse_start: 0x0, size: 1, read_efuse_cnt: 10000, efuse_ctrl: 0x30, dv_sel: EfuseAccess::Ddv, map_ptr: 0xda5708c1 };
pub static OPERATION_4: AnotherCmd = AnotherCmd { num: 3.565, adv: EfuseAccess::Dav, top: -2500, adu: EfuseAccess::Dxv, dot: ARRAY_2 };

pub static TEST_SEQUENCE: &[OperationVariant] = &[
    OperationVariant::Cmd52(&OPERATION_0),
    OperationVariant::Cmd53(&OPERATION_1),
    OperationVariant::Cmd52(&OPERATION_0),
    OperationVariant::Cmd53(&OPERATION_2),
    OperationVariant::ReadEfuse(&OPERATION_3),
    OperationVariant::AnotherCmd(&OPERATION_4),
];";
