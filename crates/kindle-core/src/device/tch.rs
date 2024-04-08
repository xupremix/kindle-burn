pub struct LibTorchCudaDevice<const N: usize>;
impl<const N: usize> crate::Sealed for LibTorchCudaDevice<N> {}
impl<const N: usize> crate::device::KindleDevice for LibTorchCudaDevice<N> {}

pub struct LibTorchCpuDevice;
impl crate::Sealed for LibTorchCpuDevice {}
impl crate::device::KindleDevice for LibTorchCpuDevice {}

pub struct LibTorchMpsDevice;
impl crate::Sealed for LibTorchMpsDevice {}
impl crate::device::KindleDevice for LibTorchMpsDevice {}

pub struct LibTorchVulkanDevice;
impl crate::Sealed for LibTorchVulkanDevice {}
impl crate::device::KindleDevice for LibTorchVulkanDevice {}

// Taking ownership
impl<const N: usize> From<LibTorchCudaDevice<N>> for crate::backend::libtorch::LibTorchDevice {
    fn from(_: LibTorchCudaDevice<N>) -> Self {
        Self::Cuda(N)
    }
}
impl From<LibTorchCpuDevice> for crate::backend::libtorch::LibTorchDevice {
    fn from(_: LibTorchCpuDevice) -> Self {
        Self::Cpu
    }
}
impl From<LibTorchMpsDevice> for crate::backend::libtorch::LibTorchDevice {
    fn from(value: LibTorchMpsDevice) -> Self {
        Self::Mps
    }
}
impl From<LibTorchVulkanDevice> for crate::backend::libtorch::LibTorchDevice {
    fn from(_: LibTorchVulkanDevice) -> Self {
        Self::Vulkan
    }
}

// Not taking ownership
impl<const N: usize> From<&LibTorchCudaDevice<N>> for crate::backend::libtorch::LibTorchDevice {
    fn from(_: &LibTorchCudaDevice<N>) -> Self {
        Self::Cuda(N)
    }
}
impl From<&LibTorchCpuDevice> for crate::backend::libtorch::LibTorchDevice {
    fn from(_: &LibTorchCpuDevice) -> Self {
        Self::Cpu
    }
}
impl From<&LibTorchMpsDevice> for crate::backend::libtorch::LibTorchDevice {
    fn from(value: &LibTorchMpsDevice) -> Self {
        Self::Mps
    }
}
impl From<&LibTorchVulkanDevice> for crate::backend::libtorch::LibTorchDevice {
    fn from(_: &LibTorchVulkanDevice) -> Self {
        Self::Vulkan
    }
}
