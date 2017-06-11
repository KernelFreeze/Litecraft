use block::model::Model;
use block::blockstate::BlockState;

pub struct Block {
    model: Model,
    state: BlockState,
    x: i32,
    y: i32,
    z: i32
}