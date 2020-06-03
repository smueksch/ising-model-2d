//! Test suite for spin representation conversions.

use ising_model_2d::spin::Spin;

#[test]
fn test_spin_from_true() {
    assert_eq!(Spin::Up, Spin::from(true));
}

#[test]
fn test_spin_from_false() {
    assert_eq!(Spin::Down, Spin::from(false));
}

#[test]
fn test_spin_from_true_ref() {
    assert_eq!(Spin::Up, Spin::from(&true));
}

#[test]
fn test_spin_from_false_ref() {
    assert_eq!(Spin::Down, Spin::from(&false));
}

#[test]
fn test_f64_from_spin_up() {
    assert_eq!(1.0, f64::from(Spin::Up));
}

#[test]
fn test_f64_from_spin_down() {
    assert_eq!(-1.0, f64::from(Spin::Down));
}

#[test]
fn test_true_to_spin_up_ref() {
    // Given:
    let val = true;

    // Then:
    assert_eq!(Spin::Up, *(&val).as_ref());
}

#[test]
fn test_false_to_spin_down_ref() {
    // Given:
    let val = false;

    // Then:
    assert_eq!(Spin::Down, *(&val).as_ref());
}
