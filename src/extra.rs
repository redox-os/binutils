use std::process::exit;
use std::error::Error;

/// Extension for Option-like types
pub trait OptionalExt {
    /// The "success" variantion of this optional type.
    type Succ;

    /// Unwrap or abort program with exit code
    fn try(self) -> Self::Succ;
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
}
