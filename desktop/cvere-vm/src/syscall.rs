// ============================================================================
// desktop/cvere-vm/src/syscall.rs
// System call handler with game integration for reality reprogramming
// ============================================================================

use std::collections::HashMap;

/// System call numbers - organized by privilege level
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u16)]
pub enum Syscall {
    // ===== RING 2 (USER MODE) - Available to all code =====
    // Console I/O (0x00-0x0F)
    Exit = 0x00,           // Exit program with code
    PrintChar = 0x01,      // Print single character
    PrintStr = 0x02,       // Print null-terminated string
    PrintHex = 0x03,       // Print hex value
    ReadChar = 0x04,       // Read character (blocking)
    ReadLine = 0x05,       // Read line of text
    ClearScreen = 0x06,    // Clear console
    SetColor = 0x07,       // Set text color
    
    // Time & Timing (0x10-0x1F)
    GetTime = 0x10,        // Get cycle count
    Sleep = 0x11,          // Sleep for N cycles
    GetRealTime = 0x12,    // Get real-world time
    SetTimer = 0x13,       // Set timer callback
    
    // Memory Management (0x20-0x2F) - User space only
    AllocMem = 0x20,       // Allocate memory in user space
    FreeMem = 0x21,        // Free allocated memory
    GetMemInfo = 0x22,     // Get memory statistics
    
    // Math & Utilities (0x30-0x3F)
    Random = 0x30,         // Get random number
    Sqrt = 0x31,           // Integer square root
    Multiply32 = 0x32,     // 32-bit multiply
    Divide32 = 0x33,       // 32-bit divide
    
    // ===== RING 1 (SUPERVISOR MODE) - Game scripting layer =====
    // Game Object Management (0x40-0x4F)
    CreateEntity = 0x40,   // Create game entity
    DestroyEntity = 0x41,  // Destroy entity
    GetEntity = 0x42,      // Get entity properties
    SetEntity = 0x43,      // Set entity properties
    MoveEntity = 0x44,     // Move entity in world
    
    // World Manipulation (0x50-0x5F)
    GetTile = 0x50,        // Get world tile data
    SetTile = 0x51,        // Modify world tile
    GetRegion = 0x52,      // Get region data
    SetRegion = 0x53,      // Modify region
    SpawnParticle = 0x54,  // Spawn particle effect
    PlaySound = 0x55,      // Play sound effect
    
    // Player Interaction (0x60-0x6F)
    GetPlayerPos = 0x60,   // Get player position
    GetPlayerStat = 0x61,  // Get player stat
    SetPlayerStat = 0x62,  // Set player stat (careful!)
    AddInventory = 0x63,   // Add item to inventory
    RemoveInventory = 0x64,// Remove from inventory
    ShowDialog = 0x65,     // Show dialog box
    
    // Quest & Story (0x70-0x7F)
    SetQuestFlag = 0x70,   // Set quest flag
    GetQuestFlag = 0x71,   // Get quest flag
    TriggerEvent = 0x72,   // Trigger story event
    SaveGame = 0x73,       // Save game state
    LoadGame = 0x74,       // Load game state
    
    // ===== RING 0 (KERNEL MODE) - Reality manipulation core =====
    // Direct Hardware Access (0x80-0x8F)
    ReadPort = 0x80,       // Read I/O port
    WritePort = 0x81,      // Write I/O port
    MapMemory = 0x82,      // Map physical memory
    UnmapMemory = 0x83,    // Unmap memory
    EnableInterrupt = 0x84,// Enable interrupt
    DisableInterrupt = 0x85,// Disable interrupt
    
    // Process Management (0x90-0x9F)
    CreateProcess = 0x90,  // Spawn new process
    KillProcess = 0x91,    // Terminate process
    SwitchProcess = 0x92,  // Context switch
    GetProcessInfo = 0x93, // Get process data
    SetPriority = 0x94,    // Set process priority
    
    // Reality Engine (0xA0-0xAF) - The DANGEROUS ones
    RealityWrite = 0xA0,   // Write to reality memory
    RealityRead = 0xA1,    // Read reality memory
    RealityCompile = 0xA2, // Compile reality code
    RealityExecute = 0xA3, // Execute reality modification
    RealityRevert = 0xA4,  // Undo reality change
    RealitySave = 0xA5,    // Save reality state
    RealityLoad = 0xA6,    // Load reality state
    RealityQuery = 0xA7,   // Query reality properties
    
    // Physics Engine (0xB0-0xBF)
    SetGravity = 0xB0,     // Modify gravity constant
    SetTimeFlow = 0xB1,    // Modify time speed
    CreatePortal = 0xB2,   // Create spatial portal
    ModifyMatter = 0xB3,   // Transmute matter
    
    // Debug & Development (0xF0-0xFF)
    Debug = 0xF0,          // Debug output
    Breakpoint = 0xF1,     // Trigger breakpoint
    DumpState = 0xF2,      // Dump VM state
    Inspect = 0xF3,        // Inspect memory
    
    Unknown = 0xFFFF,
}

impl Syscall {
    pub fn from_u16(value: u16) -> Self {
        match value {
            // User mode
            0x00 => Syscall::Exit,
            0x01 => Syscall::PrintChar,
            0x02 => Syscall::PrintStr,
            0x03 => Syscall::PrintHex,
            0x04 => Syscall::ReadChar,
            0x05 => Syscall::ReadLine,
            0x06 => Syscall::ClearScreen,
            0x07 => Syscall::SetColor,
            0x10 => Syscall::GetTime,
            0x11 => Syscall::Sleep,
            0x12 => Syscall::GetRealTime,
            0x13 => Syscall::SetTimer,
            0x20 => Syscall::AllocMem,
            0x21 => Syscall::FreeMem,
            0x22 => Syscall::GetMemInfo,
            0x30 => Syscall::Random,
            0x31 => Syscall::Sqrt,
            0x32 => Syscall::Multiply32,
            0x33 => Syscall::Divide32,
            
            // Supervisor mode (Game)
            0x40 => Syscall::CreateEntity,
            0x41 => Syscall::DestroyEntity,
            0x42 => Syscall::GetEntity,
            0x43 => Syscall::SetEntity,
            0x44 => Syscall::MoveEntity,
            0x50 => Syscall::GetTile,
            0x51 => Syscall::SetTile,
            0x52 => Syscall::GetRegion,
            0x53 => Syscall::SetRegion,
            0x54 => Syscall::SpawnParticle,
            0x55 => Syscall::PlaySound,
            0x60 => Syscall::GetPlayerPos,
            0x61 => Syscall::GetPlayerStat,
            0x62 => Syscall::SetPlayerStat,
            0x63 => Syscall::AddInventory,
            0x64 => Syscall::RemoveInventory,
            0x65 => Syscall::ShowDialog,
            0x70 => Syscall::SetQuestFlag,
            0x71 => Syscall::GetQuestFlag,
            0x72 => Syscall::TriggerEvent,
            0x73 => Syscall::SaveGame,
            0x74 => Syscall::LoadGame,
            
            // Kernel mode (Reality)
            0x80 => Syscall::ReadPort,
            0x81 => Syscall::WritePort,
            0x82 => Syscall::MapMemory,
            0x83 => Syscall::UnmapMemory,
            0x84 => Syscall::EnableInterrupt,
            0x85 => Syscall::DisableInterrupt,
            0x90 => Syscall::CreateProcess,
            0x91 => Syscall::KillProcess,
            0x92 => Syscall::SwitchProcess,
            0x93 => Syscall::GetProcessInfo,
            0x94 => Syscall::SetPriority,
            0xA0 => Syscall::RealityWrite,
            0xA1 => Syscall::RealityRead,
            0xA2 => Syscall::RealityCompile,
            0xA3 => Syscall::RealityExecute,
            0xA4 => Syscall::RealityRevert,
            0xA5 => Syscall::RealitySave,
            0xA6 => Syscall::RealityLoad,
            0xA7 => Syscall::RealityQuery,
            0xB0 => Syscall::SetGravity,
            0xB1 => Syscall::SetTimeFlow,
            0xB2 => Syscall::CreatePortal,
            0xB3 => Syscall::ModifyMatter,
            
            // Debug
            0xF0 => Syscall::Debug,
            0xF1 => Syscall::Breakpoint,
            0xF2 => Syscall::DumpState,
            0xF3 => Syscall::Inspect,
            
            _ => Syscall::Unknown,
        }
    }
    
    /// Get required privilege level for syscall
    pub fn required_privilege(&self) -> PrivilegeLevel {
        match self {
            // User mode (Ring 2)
            Syscall::Exit | Syscall::PrintChar | Syscall::PrintStr | 
            Syscall::PrintHex | Syscall::ReadChar | Syscall::ReadLine |
            Syscall::ClearScreen | Syscall::SetColor |
            Syscall::GetTime | Syscall::Sleep | Syscall::GetRealTime |
            Syscall::SetTimer | Syscall::AllocMem | Syscall::FreeMem |
            Syscall::GetMemInfo | Syscall::Random | Syscall::Sqrt |
            Syscall::Multiply32 | Syscall::Divide32 
                => PrivilegeLevel::User,
            
            // Supervisor mode (Ring 1) - Game layer
            Syscall::CreateEntity | Syscall::DestroyEntity | 
            Syscall::GetEntity | Syscall::SetEntity | Syscall::MoveEntity |
            Syscall::GetTile | Syscall::SetTile | Syscall::GetRegion |
            Syscall::SetRegion | Syscall::SpawnParticle | Syscall::PlaySound |
            Syscall::GetPlayerPos | Syscall::GetPlayerStat | 
            Syscall::SetPlayerStat | Syscall::AddInventory | 
            Syscall::RemoveInventory | Syscall::ShowDialog |
            Syscall::SetQuestFlag | Syscall::GetQuestFlag | 
            Syscall::TriggerEvent | Syscall::SaveGame | Syscall::LoadGame
                => PrivilegeLevel::Supervisor,
            
            // Kernel mode (Ring 0) - Reality manipulation
            _ => PrivilegeLevel::Kernel,
        }
    }
}

use crate::registers::PrivilegeLevel;

/// Console with extended game features
pub struct Console {
    output: Vec<char>,
    input: Vec<char>,
    color: u16,
    cursor_x: u16,
    cursor_y: u16,
}

impl Console {
    pub fn new() -> Self {
        Console {
            output: Vec::new(),
            input: Vec::new(),
            color: 0x07, // White on black
            cursor_x: 0,
            cursor_y: 0,
        }
    }

    pub fn print_char(&mut self, c: char) {
        self.output.push(c);
        if c == '\n' {
            self.cursor_x = 0;
            self.cursor_y += 1;
        } else {
            self.cursor_x += 1;
        }
        print!("{}", c);
    }
    
    pub fn print_str(&mut self, s: &str) {
        for c in s.chars() {
            self.print_char(c);
        }
    }

    pub fn print_hex(&mut self, value: u16) {
        let hex = format!("0x{:04X}", value);
        self.print_str(&hex);
    }

    pub fn read_char(&mut self) -> Option<char> {
        if self.input.is_empty() {
            None
        } else {
            Some(self.input.remove(0))
        }
    }
    
    pub fn read_line(&mut self) -> Option<String> {
        let mut line = String::new();
        while let Some(c) = self.read_char() {
            if c == '\n' {
                return Some(line);
            }
            line.push(c);
        }
        if line.is_empty() {
            None
        } else {
            Some(line)
        }
    }

    pub fn queue_input(&mut self, input: &str) {
        for c in input.chars() {
            self.input.push(c);
        }
    }

    pub fn get_output(&self) -> String {
        self.output.iter().collect()
    }

    pub fn clear_output(&mut self) {
        self.output.clear();
        self.cursor_x = 0;
        self.cursor_y = 0;
    }
    
    pub fn set_color(&mut self, color: u16) {
        self.color = color;
    }
}

/// Game entity for reality manipulation
#[derive(Debug, Clone)]
pub struct Entity {
    pub id: u16,
    pub entity_type: u16,
    pub x: i16,
    pub y: i16,
    pub z: i16,
    pub properties: HashMap<String, u16>,
}

/// Game world state
pub struct GameWorld {
    pub entities: HashMap<u16, Entity>,
    pub tiles: Vec<Vec<u16>>,  // 2D tile map
    pub quest_flags: HashMap<u16, bool>,
    pub player_x: i16,
    pub player_y: i16,
    pub player_stats: HashMap<u16, u16>,
    pub inventory: Vec<u16>,
    pub next_entity_id: u16,
}

impl GameWorld {
    pub fn new() -> Self {
        GameWorld {
            entities: HashMap::new(),
            tiles: vec![vec![0; 256]; 256],  // 256x256 world
            quest_flags: HashMap::new(),
            player_x: 128,
            player_y: 128,
            player_stats: HashMap::new(),
            inventory: Vec::new(),
            next_entity_id: 1,
        }
    }
    
    pub fn create_entity(&mut self, entity_type: u16, x: i16, y: i16) -> u16 {
        let id = self.next_entity_id;
        self.next_entity_id += 1;
        
        let entity = Entity {
            id,
            entity_type,
            x,
            y,
            z: 0,
            properties: HashMap::new(),
        };
        
        self.entities.insert(id, entity);
        id
    }
    
    pub fn destroy_entity(&mut self, id: u16) -> bool {
        self.entities.remove(&id).is_some()
    }
    
    pub fn get_entity(&self, id: u16) -> Option<&Entity> {
        self.entities.get(&id)
    }
    
    pub fn get_tile(&self, x: usize, y: usize) -> u16 {
        if x < 256 && y < 256 {
            self.tiles[y][x]
        } else {
            0
        }
    }
    
    pub fn set_tile(&mut self, x: usize, y: usize, tile: u16) {
        if x < 256 && y < 256 {
            self.tiles[y][x] = tile;
        }
    }
}

/// Reality engine state - THE DANGEROUS PART
pub struct RealityEngine {
    pub gravity: f32,
    pub time_scale: f32,
    pub reality_memory: Vec<u8>,
    pub modifications: Vec<RealityMod>,
    pub portals: Vec<Portal>,
}

#[derive(Debug, Clone)]
pub struct RealityMod {
    pub id: u16,
    pub mod_type: u16,
    pub target: u16,
    pub value: u16,
    pub active: bool,
}

#[derive(Debug, Clone)]
pub struct Portal {
    pub id: u16,
    pub x1: i16,
    pub y1: i16,
    pub x2: i16,
    pub y2: i16,
}

impl RealityEngine {
    pub fn new() -> Self {
        RealityEngine {
            gravity: 9.81,
            time_scale: 1.0,
            reality_memory: vec![0; 4096],  // 4KB reality buffer
            modifications: Vec::new(),
            portals: Vec::new(),
        }
    }
    
    pub fn reality_write(&mut self, addr: u16, value: u8) -> Result<(), String> {
        let addr = addr as usize;
        if addr < self.reality_memory.len() {
            self.reality_memory[addr] = value;
            Ok(())
        } else {
            Err("Reality write out of bounds".to_string())
        }
    }
    
    pub fn reality_read(&self, addr: u16) -> Result<u8, String> {
        let addr = addr as usize;
        if addr < self.reality_memory.len() {
            Ok(self.reality_memory[addr])
        } else {
            Err("Reality read out of bounds".to_string())
        }
    }
    
    pub fn set_gravity(&mut self, g: f32) {
        self.gravity = g;
    }
    
    pub fn set_time_flow(&mut self, scale: f32) {
        self.time_scale = scale;
    }
}

/// Memory allocator for user space
pub struct MemoryAllocator {
    heap_start: u16,
    heap_end: u16,
    allocations: HashMap<u16, u16>, // address -> size
    next_addr: u16,
}

impl MemoryAllocator {
    pub fn new(heap_start: u16, heap_end: u16) -> Self {
        MemoryAllocator {
            heap_start,
            heap_end,
            allocations: HashMap::new(),
            next_addr: heap_start,
        }
    }
    
    pub fn alloc(&mut self, size: u16) -> Option<u16> {
        if self.next_addr + size > self.heap_end {
            return None;
        }
        
        let addr = self.next_addr;
        self.allocations.insert(addr, size);
        self.next_addr += size;
        Some(addr)
    }
    
    pub fn free(&mut self, addr: u16) -> bool {
        self.allocations.remove(&addr).is_some()
    }
    
    pub fn get_stats(&self) -> (u16, u16, u16) {
        let used = self.next_addr - self.heap_start;
        let total = self.heap_end - self.heap_start;
        let free = total - used;
        (used, free, total)
    }
}