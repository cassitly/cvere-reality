// ============================================================================
// desktop/cvere-vm/src/registers.rs
// Register file with complete privilege system for game integration
// ============================================================================

use crate::syscall::Console;

/// Privilege levels - Ring-based protection
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PrivilegeLevel {
    Kernel = 0,      // Ring 0 - Reality manipulation core
    Supervisor = 1,  // Ring 1 - Game scripting layer
    User = 2,        // Ring 2 - User programs (NPCs, scripts)
}

impl PrivilegeLevel {
    pub fn can_access(&self, target: PrivilegeLevel) -> bool {
        (*self as u8) <= (target as u8)
    }
    
    pub fn from_u16(value: u16) -> Self {
        match value & 0x3 {
            0 => PrivilegeLevel::Kernel,
            1 => PrivilegeLevel::Supervisor,
            _ => PrivilegeLevel::User,
        }
    }
    
    pub fn to_u16(&self) -> u16 {
        *self as u16
    }
}

/// Register file with complete privilege and protection
pub struct RegisterFile {
    // General purpose registers R0-RF
    gp_regs: [u16; 16],
    
    // Special registers
    pub pc: u16,    // Program Counter
    pub sp: u16,    // Stack Pointer (current)
    pub lr: u16,    // Link Register
    pub sr: u16,    // Status Register
    
    // Privilege mode stack pointers (banked)
    pub kernel_sp: u16,      // Ring 0 stack
    pub supervisor_sp: u16,  // Ring 1 stack
    pub user_sp: u16,        // Ring 2 stack
    pub privilege: PrivilegeLevel,
    
    // Exception and interrupt handling
    pub exception_handler: u16,   // Exception vector
    pub interrupt_handler: u16,   // Interrupt vector
    pub saved_pc: u16,            // Saved PC on exception
    pub saved_sr: u16,            // Saved SR on exception
    pub saved_privilege: PrivilegeLevel,  // Saved privilege level
    
    // Protection and segmentation
    pub code_base: u16,      // Code segment base
    pub code_limit: u16,     // Code segment limit
    pub data_base: u16,      // Data segment base
    pub data_limit: u16,     // Data segment limit
    pub stack_base: u16,     // Stack segment base
    pub stack_limit: u16,    // Stack segment limit
    
    // Interrupt enable flags
    pub interrupts_enabled: bool,
    pub interrupt_mask: u16,
    
    // Console for syscalls
    pub console: Console,
}

impl RegisterFile {
    /// Create new register file with protection
    pub fn new() -> Self {
        RegisterFile {
            gp_regs: [0; 16],
            pc: 0,
            sp: 0xFFFE,
            lr: 0,
            sr: 0,
            
            // Stack pointers for each privilege level
            kernel_sp: 0xFFFE,      // Kernel stack at top
            supervisor_sp: 0xEFFE,  // Supervisor stack
            user_sp: 0xDFFE,        // User stack
            privilege: PrivilegeLevel::Kernel,  // Boot in kernel mode
            
            // Exception handlers
            exception_handler: 0x0010,
            interrupt_handler: 0x0020,
            saved_pc: 0,
            saved_sr: 0,
            saved_privilege: PrivilegeLevel::Kernel,
            
            // Segment registers (default: full access)
            code_base: 0x0000,
            code_limit: 0xFFFF,
            data_base: 0x0000,
            data_limit: 0xFFFF,
            stack_base: 0xD000,
            stack_limit: 0xFFFF,
            
            interrupts_enabled: true,
            interrupt_mask: 0xFFFF,
            
            console: Console::new(),
        }
    }
    
    // ========================================================================
    // REGISTER ACCESS
    // ========================================================================
    
    /// Read from general purpose register
    pub fn read_gp(&self, reg: u8) -> u16 {
        if reg >= 16 {
            return 0;
        }
        // R0 always reads as 0
        if reg == 0 {
            0
        } else {
            self.gp_regs[reg as usize]
        }
    }
    
    /// Write to general purpose register
    pub fn write_gp(&mut self, reg: u8, value: u16) {
        if reg >= 16 {
            return;
        }
        // R0 is hardwired to 0, writes are ignored
        if reg != 0 {
            self.gp_regs[reg as usize] = value;
        }
    }
    
    // ========================================================================
    // PRIVILEGE TRANSITIONS
    // ========================================================================
    
    /// Enter kernel mode (privilege escalation - ONLY via exception/interrupt)
    pub fn enter_kernel_mode(&mut self) {
        if self.privilege == PrivilegeLevel::Kernel {
            return; // Already in kernel mode
        }
        
        // Save current stack pointer
        match self.privilege {
            PrivilegeLevel::Supervisor => self.supervisor_sp = self.sp,
            PrivilegeLevel::User => self.user_sp = self.sp,
            _ => {}
        }
        
        // Switch to kernel stack
        self.sp = self.kernel_sp;
        self.privilege = PrivilegeLevel::Kernel;
    }
    
    /// Enter supervisor mode (can only drop from kernel or enter from user via syscall)
    pub fn enter_supervisor_mode(&mut self) -> Result<(), String> {
        match self.privilege {
            PrivilegeLevel::Kernel => {
                // Kernel can drop to supervisor
                self.kernel_sp = self.sp;
                self.sp = self.supervisor_sp;
                self.privilege = PrivilegeLevel::Supervisor;
                Ok(())
            }
            PrivilegeLevel::User => {
                // User must go through kernel first
                Err("Cannot enter supervisor mode from user mode".to_string())
            }
            PrivilegeLevel::Supervisor => Ok(()), // Already there
        }
    }
    
    /// Enter user mode (can drop from any higher level)
    pub fn enter_user_mode(&mut self) {
        // Save current stack pointer
        match self.privilege {
            PrivilegeLevel::Kernel => self.kernel_sp = self.sp,
            PrivilegeLevel::Supervisor => self.supervisor_sp = self.sp,
            _ => {}
        }
        
        // Switch to user stack
        self.sp = self.user_sp;
        self.privilege = PrivilegeLevel::User;
    }
    
    /// Drop privilege to target level (can only go down, not up)
    pub fn drop_privilege(&mut self, target: PrivilegeLevel) -> Result<(), String> {
        if (self.privilege as u8) > (target as u8) {
            return Err("Cannot escalate privilege with drop_privilege".to_string());
        }
        
        // Save current SP to the correct bank
        match self.privilege {
            PrivilegeLevel::Kernel => self.kernel_sp = self.sp,
            PrivilegeLevel::Supervisor => self.supervisor_sp = self.sp,
            PrivilegeLevel::User => self.user_sp = self.sp,
        }
        
        // Load target SP and set mode
        self.sp = match target {
            PrivilegeLevel::Kernel => self.kernel_sp,
            PrivilegeLevel::Supervisor => self.supervisor_sp,
            PrivilegeLevel::User => self.user_sp,
        };
        self.privilege = target;
        Ok(())
    }
    
    // ========================================================================
    // EXCEPTION AND INTERRUPT HANDLING
    // ========================================================================
    
    /// Trigger exception (always escalates to kernel)
    pub fn raise_exception(&mut self, exception_code: u16) {
        // Save state
        self.saved_pc = self.pc;
        self.saved_sr = self.sr;
        self.saved_privilege = self.privilege;
        
        // Set exception code in SR
        self.sr = (self.sr & 0x00FF) | (exception_code << 8);
        
        // Save current SP
        match self.privilege {
            PrivilegeLevel::Supervisor => self.supervisor_sp = self.sp,
            PrivilegeLevel::User => self.user_sp = self.sp,
            _ => {}
        }
        
        // Enter kernel mode and jump to handler
        self.privilege = PrivilegeLevel::Kernel;
        self.sp = self.kernel_sp;
        self.pc = self.exception_handler;
    }
    
    /// Handle interrupt (escalates to kernel if enabled)
    pub fn raise_interrupt(&mut self, irq: u16) -> bool {
        if !self.interrupts_enabled || (self.interrupt_mask & (1 << irq)) == 0 {
            return false; // Interrupt masked
        }
        
        // Save state
        self.saved_pc = self.pc;
        self.saved_sr = self.sr;
        self.saved_privilege = self.privilege;
        
        // Set IRQ number in SR
        self.sr = (self.sr & 0x00FF) | (irq << 8);
        
        // Save current SP and enter kernel
        match self.privilege {
            PrivilegeLevel::Supervisor => self.supervisor_sp = self.sp,
            PrivilegeLevel::User => self.user_sp = self.sp,
            _ => {}
        }
        
        self.privilege = PrivilegeLevel::Kernel;
        self.sp = self.kernel_sp;
        self.pc = self.interrupt_handler;
        
        true
    }
    
    /// Return from exception/interrupt (RETI instruction)
    pub fn return_from_exception(&mut self) -> Result<(), String> {
        if self.privilege != PrivilegeLevel::Kernel {
            return Err("RETI can only be called from kernel mode".to_string());
        }
        
        // Restore state
        self.pc = self.saved_pc;
        self.sr = self.saved_sr;
        
        // Save kernel SP
        self.kernel_sp = self.sp;
        
        // Restore privilege level and SP
        self.privilege = self.saved_privilege;
        self.sp = match self.privilege {
            PrivilegeLevel::Kernel => self.kernel_sp,
            PrivilegeLevel::Supervisor => self.supervisor_sp,
            PrivilegeLevel::User => self.user_sp,
        };
        
        Ok(())
    }
    
    // ========================================================================
    // PROTECTION CHECKS
    // ========================================================================
    
    /// Check if current privilege can access memory address
    pub fn can_access_memory(&self, address: u16, write: bool) -> Result<(), String> {
        // Kernel can access everything
        if self.privilege == PrivilegeLevel::Kernel {
            return Ok(());
        }
        
        // Check kernel memory protection (0x0000-0x0FFF)
        if address < 0x1000 {
            if write {
                return Err(format!("Access violation: Cannot write to kernel memory at 0x{:04X}", address));
            }
            // Read-only access allowed
            return Ok(());
        }
        
        // Check I/O memory (0xF000-0xFFFF) - kernel only
        if address >= 0xF000 {
            return Err(format!("Access violation: Cannot access I/O memory at 0x{:04X}", address));
        }
        
        // Supervisor can access game world memory (0x2000-0x7FFF)
        if self.privilege == PrivilegeLevel::Supervisor {
            if address >= 0x2000 && address < 0x8000 {
                return Ok(());
            }
        }
        
        // User mode has restricted access
        if self.privilege == PrivilegeLevel::User {
            // Can only access user heap (0x8000-0xDFFF)
            if address >= 0x8000 && address < 0xE000 {
                return Ok(());
            }
            return Err(format!("Access violation: User mode cannot access 0x{:04X}", address));
        }
        
        Ok(())
    }
    
    /// Check if can execute instruction at address
    pub fn can_execute(&self, address: u16) -> Result<(), String> {
        if address < self.code_base || address >= self.code_limit {
            return Err(format!("Execution violation: PC 0x{:04X} outside code segment", address));
        }
        Ok(())
    }
    
    // ========================================================================
    // PRIVILEGE QUERIES
    // ========================================================================
    
    pub fn is_kernel_mode(&self) -> bool {
        self.privilege == PrivilegeLevel::Kernel
    }
    
    pub fn is_supervisor_mode(&self) -> bool {
        self.privilege == PrivilegeLevel::Supervisor
    }
    
    pub fn is_user_mode(&self) -> bool {
        self.privilege == PrivilegeLevel::User
    }
    
    pub fn get_privilege_level(&self) -> PrivilegeLevel {
        self.privilege
    }
    
    // ========================================================================
    // FLAGS AND STATUS
    // ========================================================================
    
    pub fn get_flags(&self) -> StatusFlags {
        StatusFlags::from_u16(self.sr)
    }
    
    pub fn set_flags(&mut self, flags: StatusFlags) {
        // Preserve upper bits (exception code)
        let upper = self.sr & 0xFF00;
        self.sr = upper | (flags.to_u16() & 0x00FF);
    }
    
    // ========================================================================
    // RESET AND DEBUG
    // ========================================================================
    
    pub fn reset(&mut self) {
        self.gp_regs = [0; 16];
        self.pc = 0;
        self.sp = 0xFFFE;
        self.kernel_sp = 0xFFFE;
        self.supervisor_sp = 0xEFFE;
        self.user_sp = 0xDFFE;
        self.lr = 0;
        self.sr = 0;
        self.privilege = PrivilegeLevel::Kernel;
        self.saved_pc = 0;
        self.saved_sr = 0;
        self.saved_privilege = PrivilegeLevel::Kernel;
        self.interrupts_enabled = true;
        self.interrupt_mask = 0xFFFF;
    }
    
    pub fn dump(&self) -> String {
        let mut result = String::new();
        result.push_str("General Purpose Registers:\n");
        for i in 0..16 {
            result.push_str(&format!("  R{:X}: 0x{:04X}", i, self.read_gp(i)));
            if (i + 1) % 4 == 0 {
                result.push('\n');
            }
        }
        result.push_str("\nSpecial Registers:\n");
        result.push_str(&format!("  PC: 0x{:04X}\n", self.pc));
        result.push_str(&format!("  SP: 0x{:04X} (Ring {})\n", 
            self.sp, self.privilege as u8));
        result.push_str(&format!("  LR: 0x{:04X}\n", self.lr));
        result.push_str(&format!("  SR: 0x{:04X} ", self.sr));
        
        let flags = self.get_flags();
        result.push_str(&format!("[Z={} N={} C={} V={}]\n", 
            flags.zero as u8, flags.negative as u8,
            flags.carry as u8, flags.overflow as u8));
        
        result.push_str(&format!("\nPrivilege: {:?} (Ring {})\n", 
            self.privilege, self.privilege as u8));
        result.push_str(&format!("  Kernel SP:     0x{:04X}\n", self.kernel_sp));
        result.push_str(&format!("  Supervisor SP: 0x{:04X}\n", self.supervisor_sp));
        result.push_str(&format!("  User SP:       0x{:04X}\n", self.user_sp));
        
        result.push_str(&format!("\nProtection:\n"));
        result.push_str(&format!("  Code:  0x{:04X}-0x{:04X}\n", 
            self.code_base, self.code_limit));
        result.push_str(&format!("  Data:  0x{:04X}-0x{:04X}\n", 
            self.data_base, self.data_limit));
        result.push_str(&format!("  Stack: 0x{:04X}-0x{:04X}\n", 
            self.stack_base, self.stack_limit));
        
        result
    }
    
    pub fn get_console_output(&self) -> String {
        self.console.get_output()
    }
    
    pub fn queue_console_input(&mut self, input: &str) {
        self.console.queue_input(input);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct StatusFlags {
    pub zero: bool,
    pub negative: bool,
    pub carry: bool,
    pub overflow: bool,
}

impl StatusFlags {
    pub fn new() -> Self {
        StatusFlags {
            zero: false,
            negative: false,
            carry: false,
            overflow: false,
        }
    }
    
    pub fn to_u16(&self) -> u16 {
        let mut sr = 0u16;
        if self.zero { sr |= 1 << 0; }
        if self.negative { sr |= 1 << 1; }
        if self.carry { sr |= 1 << 2; }
        if self.overflow { sr |= 1 << 3; }
        sr
    }
    
    pub fn from_u16(sr: u16) -> Self {
        StatusFlags {
            zero: (sr & (1 << 0)) != 0,
            negative: (sr & (1 << 1)) != 0,
            carry: (sr & (1 << 2)) != 0,
            overflow: (sr & (1 << 3)) != 0,
        }
    }
}