
pub static TEST_FILE_H_CONTENT: &str = "\
#ifndef _TEST_SEQUENCE_H
#define _TEST_SEQUENCE_H
#include <stdint.h>
#include <stdbool.h>

enum OperationId{
   OPERATION_ID_CMD52,
   OPERATION_ID_CMD53,
   OPERATION_ID_READ_EFUSE,
   OPERATION_ID_ANOTHER_CMD,
};

enum EfuseAccess{
   EfuseAccess_DDV,
   EfuseAccess_DAV,
   EfuseAccess_DXV,
};

typedef struct{
   const bool write;
   const uint8_t fn;
   const uint32_t add;
   const uint8_t data;
}Cmd52;

typedef struct{
   const bool write;
   const uint8_t fn;
   const uint32_t add;
   const bool inc;
   const uint8_t* const data;
}Cmd53;

typedef struct{
   const uint16_t offset;
   const uint8_t efuse_start;
   const uint8_t size;
   const uint32_t read_efuse_cnt;
   const uint8_t efuse_ctrl;
   const enum EfuseAccess dv_sel;
   const uint64_t map_ptr;
}ReadEfuse;

typedef struct{
   const float num;
   const enum EfuseAccess adv;
   const int32_t top;
   const enum EfuseAccess adu;
   const int8_t* const dot;
}AnotherCmd;

union OperationVariant{
   const Cmd52* const cmd52;
   const Cmd53* const cmd53;
   const ReadEfuse* const read_efuse;
   const AnotherCmd* const another_cmd;
};

typedef struct{
   const enum OperationId id;
   const union OperationVariant variant;
}Operation;

#endif\n";
