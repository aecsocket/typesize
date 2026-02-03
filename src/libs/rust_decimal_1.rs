use crate::TypeSize;

impl TypeSize for rust_decimal_1::Decimal {
    fn extra_size(&self) -> usize {
        0
    }
}
