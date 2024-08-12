use fia::prelude::*;

#[allow(unused)]
type Sut = fia::FiaKeyDerivation;

#[test]
fn trivial() {
    assert_eq!(frobnicate(), 42)
}
