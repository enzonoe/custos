use std::{collections::HashMap, cell::RefCell};

use crate::{libs::opencl::COUNT, Matrix};

use super::CPU;

//pub static mut CPUCACHE_COUNT: usize = 0;
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Node {
    idx: usize,
    out_dims: (usize, usize),
}

impl Node {
    pub fn new(out_dims: (usize, usize)) -> Node {
        COUNT.with(|count| {
            let node = Node {
                idx: *count.borrow(),
                out_dims,
                
            };
            *count.borrow_mut() += 1;
            node
        })
    }
}

thread_local! {
    pub static CPU_CACHE: RefCell<CPUCache> = RefCell::new(CPUCache { nodes: HashMap::new() });
}

#[derive(Debug, Clone, Copy)]
pub struct CpuPtr(pub *mut usize);

unsafe impl Sync for CpuPtr {}
unsafe impl Send for CpuPtr {}

type RawInfo = (CpuPtr, (usize, usize));

#[derive(Debug)]
pub struct CPUCache {
    pub nodes: HashMap<Node, RawInfo>,
}

impl CPUCache {
    pub fn add_node<T: Default+Copy>(&mut self, node: Node) -> Matrix<T> {
        let out = Matrix::new(CPU, node.out_dims);
        self.nodes.insert(node, ( CpuPtr(out.ptr() as *mut usize), out.dims() ));
        out
    }
    
    pub fn get<T: Default+Copy>(out_dims: (usize, usize)) -> Matrix<T> {

        CPU_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            let node = Node::new(out_dims);
            let matrix_info_option = cache.nodes.get(&node);

            match matrix_info_option {
                Some(matrix_info) => Matrix::from((matrix_info.0.0 as *mut T, matrix_info.1)),
                None => cache.add_node(node)
            }
        })

        /* 

        let mut cache = CPU_CACHE.lock().unwrap();
        
        let node = Node::new(out_dims);
        let matrix_info_option = cache.nodes.get(&node);

        match matrix_info_option {
            Some(matrix_info) => Matrix::from((matrix_info.0.0 as *mut T, matrix_info.1)),
            None => cache.add_node(node)
        }

        */
    }
}
