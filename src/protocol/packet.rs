use uuid::Uuid;

pub struct Packet {
    id: i32,
    data: Vec<Field>
}
enum Field {
    Boolean(bool),
    Byte(i8),
    UByte(u8),
    Short(i16),
    UShort(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    String(String),
    Chat(String),
    VarInt(i32),
    VarLong(i32),
    //TODO: Chunk, Entity meta, slot, nbt, byte array, optx, array optx...
    Position { x: i32, y: i16, z: i32 },
    Angle(i8),
    UUID(Uuid)
}

pub enum PacketType {
    PlayIn(PlayIn),
    PlayOut(PlayOut)
}

/// Server -> Client (Play)
pub enum PlayIn {
    SpawnObject = 0x00,
    ExpOrb = 0x01,
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
    Difficulty = 0x0D,
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
    EntityLookMove = 0x26,
    EntityLook = 0x27,
    Entity = 0x28,
    VehicleMove = 0x29,
    OpenSign = 0x2A,
    PlayerAbilities = 0x2B,
    CombatEvent = 0x2C,
    PlayerList = 0x2D,
    PlayerPositionLook = 0x2E,
    UseBed = 0x2F,
    DestroyEntities = 0x30,
    RemoveEntityEffect = 0x31,
    ResourcePack = 0x32,
    Respawn = 0x33,
    EntityHeadLook = 0x34,
    WorldBorder = 0x35,
    Camera = 0x36,
    HeldItem = 0x37,
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
    PlayerListHeader = 0x47,
    CollectItem = 0x48,
    EntityTeleport = 0x49,
    EntityProperties = 0x4A,
    EntityEffect = 0x4B
}

/// Client -> Server (Play)
pub enum PlayOut {
    PlayOutTeleportConfirm = 0x00
}

impl Packet {
    /// Create a new empty packet
    pub fn new(id: i32) -> Packet {
        Packet {
            id: id,
            data: vec![]
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
}
