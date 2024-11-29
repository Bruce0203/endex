pub use endex_derive::*;

pub trait VariantIndex {
    fn variant_index(&self) -> usize;
}
