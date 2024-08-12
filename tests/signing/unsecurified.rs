use fia::prelude::*;

#[allow(unused)]
type Sut = FiaTransactionSigning;

#[test]
fn trivial() {
    assert_eq!(frobnicate(), 42)
}
