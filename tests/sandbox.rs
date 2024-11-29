use endex::VariantIndex;

#[derive(Debug, VariantIndex, Clone, Copy)]
#[repr(usize)]
pub enum TestEnum {
    A(usize, usize) = 100,
    B = 102,
    C { value: i32 } = 105,
    D = 108,
    E = 134,
    F = 841,
}

#[test]
fn test() {
    assert_eq!(TestEnum::A(1, 2).variant_index(), 100);
    assert_eq!(TestEnum::B.variant_index(), 102);
    assert_eq!(TestEnum::C { value: 123 }.variant_index(), 105);
    assert_eq!(TestEnum::D.variant_index(), 108);
    assert_eq!(TestEnum::E.variant_index(), 134);
    assert_eq!(TestEnum::F.variant_index(), 841);
}
