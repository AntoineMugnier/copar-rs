pub static TEST_FILE_C_CONTENT: &str = "\
#include \"playdisc.h\"

const uint8_t array_0[] = {0x80, 0x5};
const uint8_t array_1[] = {0x80, 0x5, 0x20, 0xfe, 0xc4, 0x31, 0x4, 0x60, 0xce};
const int8_t array_2[] = {-25, -69, 2};

const Cmd52 operation_0 = {.write = true, .ln = 1, .add = 0x1043, .data = 0x80};
const Cmd53 operation_1 = {.write = true, .ln = 1, .add = 0x1043, .inc = true, .data = array_0};
const Cmd53 operation_2 = {.write = false, .ln = 0, .add = 0x1043, .inc = true, .data = array_1};
const ReadEfuse operation_3 = {.offset = 0x5ea, .efuse_start = 0x0, .size = 1, .read_efuse_cnt = 10000, .efuse_ctrl = 0x30, .dv_sel = EfuseAccessDDV, .map_ptr = 0xda5708c1};
const AnotherCmd operation_4 = {.num = 3.565, .adv = EfuseAccessDAV, .top = -2500, .adu = EfuseAccessDXV, .dot = array_2};

const Operation test_sequence[] = {
   {.id = OPERATION_ID_CMD52, .variant={.cmd52=&operation_0}},
   {.id = OPERATION_ID_CMD53, .variant={.cmd53=&operation_1}},
   {.id = OPERATION_ID_CMD52, .variant={.cmd52=&operation_0}},
   {.id = OPERATION_ID_CMD53, .variant={.cmd53=&operation_2}},
   {.id = OPERATION_ID_READ_EFUSE, .variant={.read_efuse=&operation_3}},
   {.id = OPERATION_ID_ANOTHER_CMD, .variant={.another_cmd=&operation_4}}
};

const uint32_t test_sequence_len = sizeof(test_sequence_operations)/sizeof(Operation);";
