#[cfg(not(feature = "no-std"))]
mod std_err {
    /// A type alias for Box<dyn std::error::Error + Send + Sync>
    pub type Error = Box<dyn std::error::Error + Send + Sync>;

    /// A trait for downcasting errors.
    pub trait ErrorKind {
        /// Downcasts the error to the specified type.
        fn kind<E: std::error::Error + PartialEq + 'static>(&self) -> Option<&E>;
    }

    impl ErrorKind for Error {
        fn kind<E: std::error::Error + PartialEq + 'static>(&self) -> Option<&E> {
            self.downcast_ref::<E>()
        }
    }

    impl std::error::Error for crate::DeviceError {}
}

#[cfg(not(feature = "no-std"))]
pub use std_err::*;

/// A type alias for `Result<T, Error>`.
#[cfg(not(feature = "no-std"))]
pub type Result<T> = core::result::Result<T, self::std_err::Error>;

/// An error for no-std.
#[cfg(feature = "no-std")]
#[derive(Debug)]
pub struct Error {}

/// A type alias for `Result<T, Error>`.
#[cfg(feature = "no-std")]
pub type Result<T> = core::result::Result<T, Error>;

/// 'generic' device errors that can occur on any device.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DeviceError {
    /// Only a non-drop buffer can be converted to a CPU+OpenCL buffer.
    ConstructError,
    /// Only a CPU Buffer can be converted to a CUDA Buffer
    CPUtoCUDA,
    /// This graph can't be optimized. This indicates a bug in custos.
    GraphOptimization, // probably a bug
    /// An address was not supplied for a Network device.
    MissingAddress,
    /// Cannot create WGPU device instance.
    WGPUDeviceReturn,
    /// The 'cpu' feature is disabled. Hence this CPU can't be created.
    CPUDeviceNotAvailable,
}

impl DeviceError {
    /// Returns a string slice containing the error message.
    pub fn as_str(&self) -> &'static str {
        match self {
            DeviceError::ConstructError => {
                "Only a non-drop buffer can be converted to a CPU+OpenCL buffer."
            }
            DeviceError::CPUtoCUDA => "Only a CPU Buffer can be converted to a CUDA Buffer",
            DeviceError::GraphOptimization => {
                "This graph can't be optimized. This indicates a bug in custos."
            }
            DeviceError::MissingAddress => "An address was not supplied for a Network device.",
            DeviceError::WGPUDeviceReturn => "Cannot create WGPU device instance.",
            DeviceError::CPUDeviceNotAvailable => {
                "The 'cpu' feature is disabled. Hence this CPU can't be created."
            }
        }
    }
}

impl core::fmt::Debug for DeviceError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl core::fmt::Display for DeviceError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}
