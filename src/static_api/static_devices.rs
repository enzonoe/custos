use crate::CPU;

#[cfg(feature = "opencl")]
use crate::opencl::chosen_cl_idx;

#[cfg(feature = "cuda")]
use crate::cuda::chosen_cu_idx;

#[cfg(not(feature = "no-std"))]
thread_local! {
    pub static GLOBAL_CPU: CPU = CPU::new();
}

#[cfg(not(feature = "no-std"))]
#[inline]
pub fn static_cpu() -> &'static CPU {
    // Safety: GLOBAL_CPU should live long enough
    unsafe {
        GLOBAL_CPU
            .with(|device| device as *const CPU)
            .as_ref()
            .unwrap()
    }
}

#[cfg(feature = "no-std")]
pub static GLOBAL_CPU: Option<CPU> = None;

#[cfg(feature = "no-std")]
pub fn static_cpu() -> &'static CPU {
    if let Some(cpu) = &GLOBAL_CPU {
        cpu
    } else {
        GLOBAL_CPU = Some(CPU::new())
    }
}

#[cfg(feature = "opencl")]
thread_local! {
    pub static GLOBAL_OPENCL: crate::OpenCL = {
        crate::OpenCL::new(chosen_cl_idx()).expect("Could not create a static OpenCL device.")
    };
}

#[cfg(feature = "cuda")]
thread_local! {
    pub static GLOBAL_CUDA: crate::CUDA = {
        crate::CUDA::new(chosen_cu_idx()).expect("Could not create a static CUDA device.")
    };
}

#[cfg(feature = "opencl")]
#[inline]
pub fn static_opencl() -> &'static crate::OpenCL {
    // Safety: GLOBAL_OPENCL should live long enough
    unsafe {
        GLOBAL_OPENCL
            .with(|device| device as *const crate::OpenCL)
            .as_ref()
            .unwrap()
    }
}

#[cfg(feature = "cuda")]
#[inline]
pub fn static_cuda() -> &'static crate::CUDA {
    // Safety: GLOBAL_CUDA should live long enough
    unsafe {
        GLOBAL_CUDA
            .with(|device| device as *const crate::CUDA)
            .as_ref()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    #[cfg(not(feature = "no-std"))]
    use crate::Buffer;

    #[cfg(not(feature = "no-std"))]
    #[cfg(not(feature = "realloc"))]
    #[test]
    fn test_static_cpu_cache() {
        // for: cargo test -- --test-threads=1
        set_count(0);
        use super::static_cpu;
        use crate::{set_count, Cache, Ident};

        let cpu = static_cpu();

        let a = Buffer::from(&[1, 2, 3, 4]);
        let b = Buffer::from(&[1, 2, 3, 4]);

        let out = Cache::get::<i32, ()>(cpu, a.len(), (&a, &b));

        let cache = static_cpu().cache.borrow();
        let cached = cache
            .nodes
            .get(&Ident {
                idx: 0,
                len: out.len(),
            })
            .unwrap();

        assert_eq!(cached.ptr, out.ptr.ptr as *mut u8);
    }

    #[cfg(feature = "opencl")]
    #[test]
    fn test_to_cl() {
        let buf = Buffer::from(&[1f32, 2., 3.]);
        let mut cl = buf.to_cl();

        assert_eq!(cl.read(), vec![1., 2., 3.,]);

        cl.clear();

        assert_eq!(cl.read(), vec![0.; 3]);
    }

    #[cfg(feature = "cuda")]
    #[test]
    fn test_to_cuda() {
        let buf = Buffer::from(&[1f32, 2., 3.]);
        let mut cuda = buf.to_cuda();

        assert_eq!(cuda.read(), vec![1., 2., 3.,]);

        cuda.clear();

        assert_eq!(cuda.read(), vec![0.; 3]);
    }

    #[cfg(feature = "opencl")]
    #[test]
    fn test_to_cpu() {
        let buf = Buffer::from(&[2f32, 5., 1.]).to_cl();
        let buf = buf.to_cpu();

        assert_eq!(buf.as_slice(), &[2., 5., 1.]);
    }

    #[cfg(any(feature = "opencl", feature = "cuda"))]
    #[test]
    fn test_to_gpu() {
        use crate::buf;

        let buf = buf![2f32, 5., 1.].to_gpu();
        assert_eq!(buf.read(), vec![2., 5., 1.]);
    }

    #[cfg(feature = "cuda")]
    #[test]
    fn test_to_device_cu() {
        use crate::CUDA;

        let buf = Buffer::from(&[3f32, 1.4, 1., 2.]).to_dev::<CUDA>();

        assert_eq!(buf.read(), vec![3., 1.4, 1., 2.]);
    }

    #[cfg(feature = "opencl")]
    #[test]
    fn test_to_device_cl() {
        use crate::{buf, OpenCL};

        let buf = buf![3f32, 1.4, 1., 2.].to_dev::<OpenCL>();

        assert_eq!(buf.read(), vec![3., 1.4, 1., 2.]);
    }
}