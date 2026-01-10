"""
Example game code for "The Reality Device" - showing how to use CVERE
to create a game where you reprogram reality itself!

This demonstrates:
1. User mode code (NPC behavior)
2. Supervisor mode code (game logic)
3. Kernel mode code (reality manipulation)
"""

# ============================================================================
# EXAMPLE 1: User Mode - NPC Behavior Script
# This runs in Ring 2 (User) - safe, sandboxed code
# ============================================================================

npc_patrol = """
; NPC patrol behavior - runs in USER MODE (Ring 2)
; Can only use basic syscalls, cannot modify game world directly

start:
    LOADI R1, 0x04      ; Syscall: ReadChar (wait for input)
    LOADI R2, 0x00      ; No args
    SYSCALL             ; Make syscall
    
    ; Check if player is nearby
    LOADI R1, 0x60      ; Syscall: GetPlayerPos
    SYSCALL
    ; Returns player X in R1, Y in R2
    
    ; Calculate distance
    LOADI R3, 0x0A      ; NPC X position
    SUB   R4, R1, R3    ; Delta X
    
    ; If player close, print alert
    LOADI R5, 0x05
    SUB   R6, R4, R5
    BEQ   R6, alert
    
    ; Continue patrol
    JMP   start

alert:
    LOADI R1, 0x01      ; Syscall: PrintChar
    LOADI R2, 0x21      ; '!' character
    SYSCALL
    JMP   start
"""

# ============================================================================
# EXAMPLE 2: Supervisor Mode - Game World Modification
# This runs in Ring 1 (Supervisor) - can modify game objects
# ============================================================================

modify_world = """
; Game world modification script - SUPERVISOR MODE (Ring 1)
; Can create/destroy entities, modify tiles, trigger events

start:
    ; Create a new enemy entity
    LOADI R1, 0x40      ; Syscall: CreateEntity
    LOADI R2, 0x02      ; Entity type: 0x02 = Enemy
    LOADI R3, 0x10      ; X position
    LOADI R4, 0x10      ; Y position
    SYSCALL             ; Returns entity ID in R1
    
    ; Store entity ID
    LOADI R5, R1        ; Save ID
    
    ; Modify a tile in the world
    LOADI R1, 0x51      ; Syscall: SetTile
    LOADI R2, 0x0F      ; X coordinate
    LOADI R3, 0x0F      ; Y coordinate
    LOADI R4, 0x05      ; Tile type: 0x05 = Wall
    SYSCALL
    
    ; Spawn a particle effect
    LOADI R1, 0x54      ; Syscall: SpawnParticle
    LOADI R2, 0x10      ; X position
    LOADI R3, 0x10      ; Y position
    LOADI R4, 0x01      ; Particle type: explosion
    SYSCALL
    
    ; Set a quest flag
    LOADI R1, 0x70      ; Syscall: SetQuestFlag
    LOADI R2, 0x01      ; Flag ID: quest_started
    LOADI R3, 0x01      ; Value: true
    SYSCALL
    
    ; Show dialog
    LOADI R1, 0x65      ; Syscall: ShowDialog
    LOADI R2, 0x1000    ; Pointer to dialog string
    SYSCALL
    
    HALT
"""

# ============================================================================
# EXAMPLE 3: Kernel Mode - REALITY MANIPULATION
# This runs in Ring 0 (Kernel) - THE DANGEROUS STUFF
# Only accessible when you find the Reality Device in-game
# ============================================================================

reality_hack = """
; REALITY MANIPULATION CODE - KERNEL MODE ONLY (Ring 0)
; WARNING: This can break the game world if misused!
; Player obtains this power late in the game

reality_start:
    ; First, verify we're in kernel mode
    ; (In real implementation, this would be checked by VM)
    
    ; === MODIFY GRAVITY ===
    LOADI R1, 0xB0      ; Syscall: SetGravity
    LOADI R2, 0x05      ; New gravity value (5.0 instead of 9.81)
    LOADI R3, 0x00      ; Fractional part
    SYSCALL
    ; The world's gravity just changed!
    
    ; === MODIFY TIME FLOW ===
    LOADI R1, 0xB1      ; Syscall: SetTimeFlow
    LOADI R2, 0x02      ; 2x speed (time flows twice as fast)
    SYSCALL
    ; Everything now moves in slow motion!
    
    ; === CREATE A SPATIAL PORTAL ===
    LOADI R1, 0xB2      ; Syscall: CreatePortal
    LOADI R2, 0x10      ; Portal entrance X
    LOADI R3, 0x10      ; Portal entrance Y
    LOADI R4, 0x50      ; Portal exit X
    LOADI R5, 0x50      ; Portal exit Y
    SYSCALL             ; Returns portal ID in R1
    
    ; === READ REALITY MEMORY ===
    ; This lets you peek at the underlying game code!
    LOADI R1, 0xA1      ; Syscall: RealityRead
    LOADI R2, 0x0100    ; Address in reality memory
    SYSCALL             ; Returns value in R1
    
    ; === WRITE REALITY MEMORY ===
    ; This REWRITES the game's code while it's running!
    LOADI R1, 0xA0      ; Syscall: RealityWrite
    LOADI R2, 0x0100    ; Address
    LOADI R3, 0x42      ; New value (inject our code)
    SYSCALL
    ; You just modified the game's executable code!
    
    ; === COMPILE REALITY CODE ===
    ; Take assembly code and compile it into reality
    LOADI R1, 0xA2      ; Syscall: RealityCompile
    LOADI R2, 0x2000    ; Pointer to source code
    LOADI R3, 0x100     ; Length
    SYSCALL             ; Returns compiled code address in R1
    
    ; === EXECUTE REALITY MODIFICATION ===
    ; Run the compiled code to change reality
    LOADI R1, 0xA3      ; Syscall: RealityExecute
    LOADI R2, R1        ; Code address from compile
    SYSCALL
    ; Reality has been permanently altered!
    
    ; === SAVE THIS REALITY ===
    LOADI R1, 0xA5      ; Syscall: RealitySave
    LOADI R2, 0x01      ; Save slot 1
    SYSCALL
    
    HALT

; If something goes wrong, revert reality
panic:
    LOADI R1, 0xA4      ; Syscall: RealityRevert
    SYSCALL             ; Undo last change
    HALT
"""

# ============================================================================
# EXAMPLE 4: Complete Game Scenario - "Discovering the Device"
# ============================================================================

game_scenario = """
; THE REALITY DEVICE - Game Scenario
; Player finds an ancient terminal that can reprogram reality

main_game:
    ; Initialize game in SUPERVISOR mode
    LOADI R1, 0x40      ; CreateEntity
    LOADI R2, 0xFF      ; Entity type: Player
    LOADI R3, 0x80      ; Start X: 128
    LOADI R4, 0x80      ; Start Y: 128
    SYSCALL
    
    ; Place the Reality Device in the world (hidden)
    LOADI R1, 0x40      ; CreateEntity
    LOADI R2, 0xFE      ; Entity type: Reality Device
    LOADI R3, 0xFF      ; X: 255 (far corner)
    LOADI R4, 0xFF      ; Y: 255
    SYSCALL
    LOADI R9, R1        ; Save device entity ID
    
game_loop:
    ; Get player input
    LOADI R1, 0x04      ; ReadChar
    SYSCALL
    LOADI R8, R1        ; Save input
    
    ; Check if player pressed 'E' (interact)
    LOADI R7, 0x45      ; 'E' key
    SUB   R6, R8, R7
    BEQ   R6, check_device
    
    ; Otherwise, move player based on WASD
    ; ... movement code here ...
    JMP   game_loop

check_device:
    ; Check if player is near Reality Device
    LOADI R1, 0x60      ; GetPlayerPos
    SYSCALL
    ; R1 = player X, R2 = player Y
    
    ; Check if at device location (255, 255)
    LOADI R3, 0xFF
    SUB   R4, R1, R3
    BNE   R4, game_loop ; Not at X
    
    SUB   R5, R2, R3
    BNE   R5, game_loop ; Not at Y
    
    ; Player found the device!
    JMP   activate_device

activate_device:
    ; Show discovery message
    LOADI R1, 0x65      ; ShowDialog
    LOADI R2, msg_found ; "You found the Reality Device!"
    SYSCALL
    
    ; Give player kernel privileges (!!!)
    ; In VM implementation, this would be:
    ; vm.registers.enter_kernel_mode()
    
    ; Now player can run reality hacks!
    JMP   reality_menu

reality_menu:
    ; Show what player can hack
    LOADI R1, 0x02      ; PrintStr
    LOADI R2, menu_text
    SYSCALL
    
    ; Read choice
    LOADI R1, 0x04
    SYSCALL
    
    ; Choice 1: Modify gravity
    LOADI R2, 0x31      ; '1'
    SUB   R3, R1, R2
    BEQ   R3, hack_gravity
    
    ; Choice 2: Modify time
    LOADI R2, 0x32      ; '2'
    SUB   R3, R1, R2
    BEQ   R3, hack_time
    
    ; Choice 3: Create portal
    LOADI R2, 0x33      ; '3'
    SUB   R3, R1, R2
    BEQ   R3, hack_portal
    
    ; Choice 4: Direct code injection
    LOADI R2, 0x34      ; '4'
    SUB   R3, R1, R2
    BEQ   R3, hack_reality
    
    JMP   game_loop

hack_gravity:
    LOADI R1, 0xB0      ; SetGravity
    LOADI R2, 0x00      ; Zero gravity!
    SYSCALL
    
    LOADI R1, 0x02      ; Print success
    LOADI R2, msg_gravity
    SYSCALL
    JMP   reality_menu

hack_time:
    LOADI R1, 0xB1      ; SetTimeFlow
    LOADI R2, 0x0A      ; 10x speed
    SYSCALL
    
    LOADI R1, 0x02
    LOADI R2, msg_time
    SYSCALL
    JMP   reality_menu

hack_portal:
    LOADI R1, 0xB2      ; CreatePortal
    LOADI R2, 0x10      ; From current location
    LOADI R3, 0x10
    LOADI R4, 0xF0      ; To anywhere!
    LOADI R5, 0xF0
    SYSCALL
    
    LOADI R1, 0x02
    LOADI R2, msg_portal
    SYSCALL
    JMP   reality_menu

hack_reality:
    ; Let player write assembly code to inject
    LOADI R1, 0x02
    LOADI R2, msg_inject
    SYSCALL
    
    ; Read code from player
    LOADI R1, 0x05      ; ReadLine
    SYSCALL
    ; ... store in buffer ...
    
    ; Compile it
    LOADI R1, 0xA2      ; RealityCompile
    LOADI R2, buffer
    SYSCALL
    
    ; Execute it (DANGEROUS!)
    LOADI R1, 0xA3      ; RealityExecute
    LOADI R2, R1
    SYSCALL
    
    ; Save this modified reality
    LOADI R1, 0xA5      ; RealitySave
    LOADI R2, 0x01
    SYSCALL
    
    LOADI R1, 0x02
    LOADI R2, msg_injected
    SYSCALL
    JMP   reality_menu

; String data
msg_found:      .ascii "You found the Reality Device!\\n\\0"
menu_text:      .ascii "1: Modify Gravity\\n2: Modify Time\\n3: Create Portal\\n4: Inject Code\\n\\0"
msg_gravity:    .ascii "Gravity modified!\\n\\0"
msg_time:       .ascii "Time flow altered!\\n\\0"
msg_portal:     .ascii "Portal created!\\n\\0"
msg_inject:     .ascii "Enter code to inject:\\n\\0"
msg_injected:   .ascii "Reality has been reprogrammed!\\n\\0"
buffer:         .space 256
"""

# ============================================================================
# GAME DESIGN NOTES
# ============================================================================

game_design = """
GAME CONCEPT: "The Reality Device"

STORY:
- You find an ancient terminal in an abandoned lab
- It's a reality manipulation device that can reprogram the world
- The device uses CVERE assembly language
- As you learn more assembly, you gain more power

PROGRESSION:
1. Early game: User mode only (Ring 2)
   - Can only interact with NPCs
   - Limited console I/O
   - Safe, sandboxed environment

2. Mid game: Supervisor mode (Ring 1)
   - Find a "Game Master Module"
   - Can create/destroy entities
   - Modify the world map
   - Trigger quest events
   - Essentially become a game developer while playing

3. Late game: Kernel mode (Ring 0)
   - Find the "Reality Core"
   - Can modify physics (gravity, time)
   - Create spatial portals
   - DIRECT CODE INJECTION into the running game
   - Save/load reality states
   - Ultimate power but also ultimate responsibility

GAMEPLAY MECHANICS:
- Write assembly code in-game at terminals
- Code you write becomes part of the world
- NPCs can also write code (they're programs too!)
- Find "code fragments" that teach new instructions
- Debug your code to fix reality glitches
- Boss fights = debugging corrupted code
- Final boss = the anti-reality virus

PUZZLE EXAMPLES:
1. Bridge is broken -> write code to create a portal across
2. Enemy is invincible -> modify its health in memory
3. Door requires key -> rewrite the door's check code
4. Time limit on level -> slow down time
5. Can't reach platform -> reduce gravity

META-NARRATIVE:
- The game world is literally a VM
- You're learning that you're IN a simulation
- The Reality Device lets you see/edit the source code
- Philosophical questions about free will and determinism
- If you can rewrite reality, what's real?

MULTIPLAYER IDEA:
- Share "reality mods" (assembly code) with other players
- Download reality patches from the "Reality Repository"
- Speedruns = who can hack reality fastest
- PvP = code injection battles!
"""

print("CVERE Game Examples Generated!")
print("These show how to use the privilege system for game development.")
print("See the comments in each section for details!")