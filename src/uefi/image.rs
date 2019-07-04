pub enum SystemType {
    Application = 10,
    BootServiceDriver = 11,
    RuntimeDriver = 12,
}

pub enum MachineType {
    IA32 = 0x014c,
    EBC = 0x0200,
    x64 = 0x0EBC,
    IA64 = 0x8664,
    ARMTHUMB_MIXED = 0x01C2,
    AARCH64 = 0xAA64,
    RISCV32 = 0x5032,
    RISCV64 = 0x5064,
    RISCV128 = 0x5128,
}
