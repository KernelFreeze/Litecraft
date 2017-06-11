use block::block::Block;

pub struct Chunk {
    sections: [Option<ChunkSection>; 16],
    x: i32,
    z: i32
}
pub struct ChunkSection {
    blocks: [Option<Block>; 16 * 16]
}