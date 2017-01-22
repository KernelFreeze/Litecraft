use uuid::Uuid;
use std::io;

pub struct Packet {
    id: i32,
    data: Vec<Field>,
    buf: Vec<u8>
}

#[derive(Debug, Clone)]
enum Field {
    Boolean(bool),
    Byte(i8),
    UByte(u8),
    Short(i16),
    UShort(u16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    String(String),
    Chat(String),
    VarInt(i32),
    VarLong(i64),
    //TODO: Chunk Section, Entity Metadata, Slot, NBT Tag, Byte Array, Optional X, Array of X, X Enum
    Position {
        x: i32,
        y: i16,
        z: i32
    },
    Angle(i8),
    UUID(Uuid)
}

pub enum PacketType {
    HandshakingOut(HandshakingOut),
    PlayIn(PlayIn),
    PlayOut(PlayOut),
    StatusIn(StatusIn),
    StatusOut(StatusOut),
    LoginIn(LoginIn),
    LoginOut(LoginOut)
}

/// Client -> Server (Play)
pub enum HandshakingOut {
    Handshake = 0x00,
    LegacyServerListPing = 0xFE
}

/// Server -> Client (Play)
pub enum PlayIn {
    SpawnObject = 0x00,
    SpawnExpOrb = 0x01,
    SpawnGlobalEntity = 0x02,
    SpawnMob = 0x03,
    SpawnPainting = 0x04,
    SpawnPlayer = 0x05,
    Animation = 0x06,
    Statistics = 0x07,
    BlockBreakAnimation = 0x08,
    UpdateBlockEntity = 0x09,
    BlockAction = 0x0A,
    BlockChange = 0x0B,
    BossBar = 0x0C,
    ServerDifficulty = 0x0D,
    TabComplete = 0x0E,
    ChatMessage = 0x0F,
    MultiBlockChange = 0x10,
    ConfirmTransaction = 0x11,
    CloseWindow = 0x12,
    OpenWindow = 0x13,
    WindowItems = 0x14,
    WindowProperty = 0x15,
    SetSlot = 0x16,
    SetCooldown = 0x17,
    PluginMessage = 0x18,
    NamedSoundEffect = 0x19,
    Disconnect = 0x1A,
    EntityStatus = 0x1B,
    Explosion = 0x1C,
    UnloadChunk = 0x1D,
    ChangeGameState = 0x1E,
    KeepAlive = 0x1F,
    ChunkData = 0x20,
    Effect = 0x21,
    Particle = 0x22,
    JoinGame = 0x23,
    Map = 0x24,
    EntityRelativeMove = 0x25,
    EntityLookRelativeMove = 0x26,
    EntityLook = 0x27,
    Entity = 0x28,
    VehicleMove = 0x29,
    OpenSignEditor = 0x2A,
    PlayerAbilities = 0x2B,
    CombatEvent = 0x2C,
    PlayerListItem = 0x2D,
    PlayerPositionLook = 0x2E,
    UseBed = 0x2F,
    DestroyEntities = 0x30,
    RemoveEntityEffect = 0x31,
    ResourcePackSend = 0x32,
    Respawn = 0x33,
    EntityHeadLook = 0x34,
    WorldBorder = 0x35,
    Camera = 0x36,
    HeldItemChange = 0x37,
    DisplayScoreboard = 0x38,
    EntityMetadata = 0x39,
    AttachEntity = 0x3A,
    EntityVelocity = 0x3B,
    EntityEquipment = 0x3C,
    SetExp = 0x3D,
    UpdateHealth = 0x3E,
    ScoreboardObjective = 0x3F,
    SetPassengers = 0x40,
    Teams = 0x41,
    UpdateScore = 0x42,
    SpawnPosition = 0x43,
    TimeUpdate = 0x44,
    Title = 0x45,
    SoundEffect = 0x46,
    PlayerListHeaderFooter = 0x47,
    CollectItem = 0x48,
    EntityTeleport = 0x49,
    EntityProperties = 0x4A,
    EntityEffect = 0x4B
}

/// Client -> Server (Play)
pub enum PlayOut {
    TeleportConfirm = 0x00,
    TabComplete = 0x01,
    ChatMessage = 0x02,
    ClientStatus = 0x03,
    ClientSettings = 0x04,
    ConfirmTransaction = 0x05,
    EnchantItem = 0x06,
    ClickWindow = 0x07,
    CloseWindow = 0x08,
    PluginMessage = 0x09,
    UseEntity = 0x0A,
    KeepAlive = 0x0B,
    PlayerPosition = 0x0C,
    PlayerPositionLook = 0x0D,
    PlayerLook = 0x0E,
    Player = 0x0F,
    VehicleMove = 0x10,
    SteerBoat = 0x11,
    PlayerAbilities = 0x12,
    PlayerDigging = 0x13,
    EntityAction = 0x14,
    SteerVehicle = 0x15,
    ResourcePackStatus = 0x16,
    HeldItemChange = 0x17,
    CreativeInventoryAction = 0x18,
    UpdateSign = 0x19,
    Animation = 0x1A,
    Spectate = 0x1B,
    PlayerBlockPlacement = 0x1C,
    UseItem = 0x1D
}

/// Server -> Client (Status)
pub enum StatusIn {
    Response = 0x00,
    Pong = 0x01
}

/// Client -> Server (Status)
pub enum StatusOut {
    Request = 0x00,
    Ping = 0x01
}

/// Server -> Client (Login)
pub enum LoginIn {
    Disconnect = 0x00,
    EncryptionRequest = 0x01,
    LoginSuccess = 0x02,
    SetCompression = 0x03
}

/// Client -> Server (Login)
pub enum LoginOut {
    LoginStart = 0x00,
    EncryptionResponse = 0x01
}

impl Packet {
    /// Create a new empty packet
    pub fn new(id: i32, buffer: Vec<u8>) -> Packet {
        Packet {
            id: id,
            data: vec![],
            buf: buffer
        }
    }

    // Decode a byte array into a packet
    //pub fn decode(data: &[u8]) -> Option<Packet> {}

    // Encode the packet into a byte array
    //pub fn encode(&self) -> &[u8] {}

    /// Get data field
    pub fn get_data(&self) -> &[Field] {
        &self.data
    }

    /// Get mutable data field
    pub fn get_data_mut(&mut self) -> &mut [Field] {
        &mut self.data
    }

    /// Read a VarInt, return the value and add it to the packet fields
    pub fn read_varint(&mut self) -> Result<i32, &'static str> {
        const PART: u8 = 0x7F;
        let mut size = 0;
        let mut val = 0u32;
        for b in &self.buf {
            val |= ((b & PART) << (size * 7)) as u32;
            size += 1;
            if size > 5 {
                return Result::Err("VarInt too big");
            }
            if (b & 0x80) == 0 {
                break
            }
        }

        Result::Ok(val as i32)
    }

    /// Write a VarInt, return an error if there is one
    pub fn write_varint(&mut self) -> Result<(), Error> {
        const PART: u32 = 0x7F;
        let mut val = self.0 as u32;
        loop {
            if (val & !PART) == 0 {
                self.buf.write_u8(val as u8)?;
                return Result::Ok(());
            }
            self.buf.write_u8(((val & PART) | 0x80) as u8)?;
            val >>= 7;
        }
    }
}
