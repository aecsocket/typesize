use crate::TypeSize;

impl TypeSize for uuid_1::Uuid {
    fn extra_size(&self) -> usize {
        0
    }
}
