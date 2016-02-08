use std::process::exit;
use std::error::Error;

/// Extension for Option-like types
pub trait OptionalExt {
    /// The "success" variantion of this optional type.
    type Succ;

    /// Unwrap or abort program with exit code
    fn try(self) -> Self::Succ;

    /// An unwrapping where the fail-case is not checked and threaten as statical unreachable.
    unsafe fn unchecked_unwrap(self) -> Self::Succ;
}

impl<T, U: Error> OptionalExt for Result<T, U> {
    type Succ = T;

    fn try(self) -> T {
        match self {
            Ok(succ) => succ,
            Err(e) => {
                println!("error: {}", e.description());
                exit(1);
            },
        }
    }

    unsafe fn unchecked_unwrap(self) -> T {
        if let Ok(x) = self {
            x
        } else {
            unreachable()
        }
    }
}

/// A hint which is threaten as statical unreachable in release mode, and panic (unreachable!())
/// in debug mode.
#[cfg(debug)]
pub unsafe fn unreachable() -> ! {
    unreachable!();
}


/// A hint which is threaten as statical unreachable in release mode, and panic (unreachable!())
/// in debug mode.
#[cfg(not(debug))]
pub unsafe fn unreachable() -> ! {
    use std::intrinsics::unreachable;

    unreachable();
}

#[macro_export]
macro_rules! try_some {
    ($x:expr) => {
        if let Some(x) = $x {
            x
        } else {
            return None;
        }
    };
    ($x:expr => $y:expr) => {
        if let Some(x) = $x {
            x
        } else {
            return $y;
        }
    };
}
