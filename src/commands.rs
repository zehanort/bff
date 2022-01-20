use anyhow::Result;

pub enum BaseUneCommands {
    Space,
    Not,
    StringMode,
    Trampoline,
    Pop,
    Remainder,
    InputInteger,
    FetchCharacter,
    LoadSemantics,
    UnloadSemantics,
    Multiply,
    Add,
    OutputCharacter,
    Subtract,
    OutputInteger,
    Divide,
    Push,
    Duplicate,
    JumpOver,
    West,
    East,
    Away,
    Stop,
    FingerprintCall,
    Swap,
    EastWestIf,
    GreaterThan,
    Get,
    JumpForward,
    Iterate,
    ClearStack,
    Put,
    Quit,
    Reflect,
    StoreCharacter,
    StackUnderStack,
    AbsoluteDelta,
    GetSysInfo,
    NoOperation,
    BeginBlock,
    EndBlock,
    InputCharacter,
}

pub enum BaseBeCommands {
    Base(BaseUneCommands),
    Left,
    Right,
    North,
    South,
    Compare,
    NorthSouthIf,
}

pub enum BaseTreCommands {
    Base(BaseBeCommands),
    High,
    Low,
    HighLow,
}

pub enum ConcurrentCommands {
    Split,
}

pub enum FilesystemCommands {
    Execute,
    InputFile,
    OutputFile,
}
