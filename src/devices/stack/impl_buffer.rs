use super::{stack_array::StackArray, stack_device::Stack};
use crate::{BufFlag, Buffer, Node};

/*impl<'a, T, const N: usize> From<[T; N]> for Buffer<'a, T, Stack, N> {
    fn from(array: [T; N]) -> Self {
        Buffer {
            ptr: StackArray::new(array),
            len: N,
            device: Some(&Stack),
            flag: BufFlag::None,
            node: Node::default(),
        }
    }
}

impl<'a, T: Copy, const N: usize> From<&[T; N]> for Buffer<'a, T, Stack, N> {
    fn from(array: &[T; N]) -> Self {
        Buffer {
            ptr: StackArray::new(*array),
            len: N,
            device: Some(&Stack),
            flag: BufFlag::None,
            node: Node::default(),
        }
    }
}*/

impl<'a, T, const N: usize> From<(Stack, [T; N])> for Buffer<'a, T, Stack, N> {
    fn from(array: (Stack, [T; N])) -> Self {
        Buffer {
            ptr: StackArray::new(array.1),
            len: N,
            device: Some(&Stack),
            flag: BufFlag::None,
            node: Node::default(),
        }
    }
}

impl<'a, T, const N: usize> From<(&Stack, [T; N])> for Buffer<'a, T, Stack, N> {
    fn from(array: (&Stack, [T; N])) -> Self {
        Buffer {
            ptr: StackArray::new(array.1),
            len: N,
            device: Some(&Stack),
            flag: BufFlag::None,
            node: Node::default(),
        }
    }
}

impl<'a, T: Copy, const N: usize> From<(Stack, &[T; N])> for Buffer<'a, T, Stack, N> {
    fn from(array: (Stack, &[T; N])) -> Self {
        Buffer {
            ptr: StackArray::new(*array.1),
            len: N,
            device: Some(&Stack),
            flag: BufFlag::None,
            node: Node::default(),
        }
    }
}

impl<'a, T: Copy, const N: usize> From<(&Stack, &[T; N])> for Buffer<'a, T, Stack, N> {
    fn from(array: (&Stack, &[T; N])) -> Self {
        Buffer {
            ptr: StackArray::new(*array.1),
            len: N,
            device: Some(&Stack),
            flag: BufFlag::None,
            node: Node::default(),
        }
    }
}
