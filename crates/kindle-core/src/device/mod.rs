#[cfg(feature = "wgpu")]
mod wgpu;
#[cfg(feature = "wgpu")]
pub use wgpu::*;

#[allow(private_bounds)]
pub trait KindleDevice: crate::Sealed {}
