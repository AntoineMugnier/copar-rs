pub static TEST_FILE_CS_CONTENT: &str = "\
namespace GeneratedTestSequence
{
    public enum OperationId {
        Cmd52,
        Cmd53,
        ReadEfuse,
        AnotherCmd,
    }

    public readonly struct Operation {
        public readonly OperationId Id;
        public readonly object Variant;

        public Operation(OperationId id, object variant) {
            Id = id;
            Variant = variant;
        }
    }

    public enum EfuseAccess {
        DDV,
        DAV,
        DXV,
    }

    public struct Cmd52 {
        public bool write { get; set; }
        public byte ln { get; set; }
        public uint add { get; set; }
        public byte data { get; set; }
    }

    public struct Cmd53 {
        public bool write { get; set; }
        public byte ln { get; set; }
        public uint add { get; set; }
        public bool inc { get; set; }
        public byte[] data { get; set; }
    }

    public struct ReadEfuse {
        public ushort offset { get; set; }
        public byte efuse_start { get; set; }
        public byte size { get; set; }
        public uint read_efuse_cnt { get; set; }
        public byte efuse_ctrl { get; set; }
        public EfuseAccess dv_sel { get; set; }
        public ulong map_ptr { get; set; }
    }

    public struct AnotherCmd {
        public float num { get; set; }
        public EfuseAccess adv { get; set; }
        public int top { get; set; }
        public EfuseAccess adu { get; set; }
        public sbyte[] dot { get; set; }
    }

    public static class TestSequenceConstants
    {
        public static readonly byte[] Array0 =  { 0x80, 0x5 };
        public static readonly byte[] Array1 =  { 0x80, 0x5, 0x20, 0xfe, 0xc4, 0x31, 0x4, 0x60, 0xce };
        public static readonly sbyte[] Array2 =  { -25, -69, 2 };

        public static readonly Cmd52 Operation0 = new Cmd52 { write = true, ln = 1, add = 0x1043, data = 0x80 };
        public static readonly Cmd53 Operation1 = new Cmd53 { write = true, ln = 1, add = 0x1043, inc = true, data = Array0 };
        public static readonly Cmd53 Operation2 = new Cmd53 { write = false, ln = 0, add = 0x1043, inc = true, data = Array1 };
        public static readonly ReadEfuse Operation3 = new ReadEfuse { offset = 0x5ea, efuse_start = 0x0, size = 1, read_efuse_cnt = 10000, efuse_ctrl = 0x30, dv_sel = EfuseAccess.DDV, map_ptr = 0xda5708c1 };
        public static readonly AnotherCmd Operation4 = new AnotherCmd { num = 3.565f, adv = EfuseAccess.DAV, top = -2500, adu = EfuseAccess.DXV, dot = Array2 };

        public static readonly Operation[] TestSequence = {
            new Operation(OperationId.Cmd52, Operation0),
            new Operation(OperationId.Cmd53, Operation1),
            new Operation(OperationId.Cmd52, Operation0),
            new Operation(OperationId.Cmd53, Operation2),
            new Operation(OperationId.ReadEfuse, Operation3),
            new Operation(OperationId.AnotherCmd, Operation4),
        };
    }
}";
