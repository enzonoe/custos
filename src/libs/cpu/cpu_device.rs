use std::fmt::Debug;

use crate::{Device, VecRead, Buffer, BaseDevice, AsDev};

#[derive(Debug, Clone, Copy)]
pub struct CPU;

impl CPU {
    pub fn drop<T>(buf: Buffer<T>) {
        unsafe {    
            drop(Box::from_raw(buf.ptr));
        }
    }
}

impl <T>BaseDevice<T> for CPU {
    fn add(&self, lhs: Buffer<T>, rhs: Buffer<T>) {
        println!("add {:?} with {:?}", lhs.ptr, rhs.ptr);
    }
}

impl AsDev for CPU {
    fn as_dev(&self) -> crate::Dev {
        crate::Dev::new(None)
    }
}

impl Device for CPU {
    fn alloc<T: Default+Copy>(&self, len: usize) -> *mut T {
        Box::into_raw(vec![T::default(); len].into_boxed_slice()) as *mut T
    }

    fn from_data<T: Clone>(&self, data: &[T]) -> *mut T {
        Box::into_raw(data.to_vec().into_boxed_slice()) as *mut T
    }
}

impl <T: Clone>VecRead<T> for CPU {
    fn read(&self, buf: &crate::Buffer<T>) -> Vec<T> {
        unsafe {
            std::slice::from_raw_parts(buf.ptr, buf.len).to_vec()
        }
    }
}