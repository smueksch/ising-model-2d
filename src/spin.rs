//! Enum representation of spin to enable type-checked conversion
//! between various interpretations of spins.

/// Enum representing possible spin states.
#[derive(Debug, PartialEq, Eq)]
pub enum Spin {
    Up,
    Down,
}

impl From<bool> for Spin {
    /// Convert boolean spin representation to enum representation.
    fn from(bool_repr: bool) -> Self {
        if bool_repr {
            Spin::Up
        } else {
            Spin::Down
        }
    }
}

impl From<&bool> for Spin {
    /// Convert boolean spin representation reference to enum representation.
    fn from(bool_repr: &bool) -> Self {
        if *bool_repr {
            Spin::Up
        } else {
            Spin::Down
        }
    }
}

impl From<Spin> for bool {
    /// Convert enum spin representation to boolean representation.
    fn from(spin: Spin) -> Self {
        match spin {
            Spin::Up => true,
            Spin::Down => false,
        }
    }
}

impl From<Spin> for f64 {
    fn from(spin: Spin) -> Self {
        match spin {
            Spin::Up => 1.0,
            Spin::Down => -1.0,
        }
    }
}

impl AsRef<Spin> for bool {
    /// Convert a boolean spin representation reference to enum one.
    fn as_ref(&self) -> &Spin {
        if *self {
            &Spin::Up
        } else {
            &Spin::Down
        }
    }
}
