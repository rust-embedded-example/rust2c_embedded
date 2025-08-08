#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
extern crate alloc;

use core::panic::PanicInfo;

use freertos_rust::{FreeRtosAllocator, Task, TaskPriority, CurrentTask, Duration, FreeRtosUtils};
use core::ffi::{c_char, c_int};
use core::alloc::Layout;
use alloc::boxed::Box;

use alloc::vec::Vec;
use alloc::vec;

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    //asm::bkpt();
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[global_allocator]
static GLOBAL: FreeRtosAllocator = FreeRtosAllocator;

// 引入 C 语言的 rust_log_printf 函数
// cbindgen: ignore
extern "C" {
    fn rust_log_printf(format: *const c_char, ...) -> c_int;
}


#[repr(C)]
#[derive(Clone)]
pub struct Unit {
    pub id: usize,
    // 其他字段可以根据需要添加，例如：
    // data: String,
}

#[repr(C)]
#[derive(Clone)]
pub struct EdgeData {
    pub weight: f32,
}

#[repr(C)]
pub struct Graph {
    pub adjacency_list: Vec<Vec<(usize, EdgeData)>>, // 邻接表：每个节点的邻居列表，包含邻居 ID 和边数据
    pub node_units: Vec<Vec<Unit>>,                 // 每个节点上的单元列表
    pub unit_to_node: Vec<Option<usize>>,           // 单元到节点的映射：索引是单元 ID，值是节点 ID
}

#[allow(dead_code)]
impl Graph {
    /// 创建一个新的图，指定节点数量和最大单元 ID
    fn new(num_nodes: usize, max_unit_id: usize) -> Self {
        Graph {
            adjacency_list: vec![vec![]; num_nodes],
            node_units: vec![vec![]; num_nodes],
            unit_to_node: vec![None; max_unit_id + 1], // 假设单元 ID 从 0 开始
        }
    }

    /// 添加一条边（无向图，因此同时添加反向边）
    fn add_edge(&mut self, from: usize, to: usize, edge_data: EdgeData) {
        unsafe {
            rust_log_printf(b"add_edge: from %d to %d\n\0".as_ptr() as *const c_char, from, to);
        }
        if from < self.adjacency_list.len() && to < self.adjacency_list.len() {
            self.adjacency_list[from].push((to, edge_data.clone()));
            self.adjacency_list[to].push((from, edge_data)); // 无向图
        } else {
            unsafe {
                    rust_log_printf(
                b"Error: Node index out of bounds\0".as_ptr() as *const c_char,
            );}
        }
    }

    /// 将单元添加到指定节点
    fn add_unit_to_node(&mut self, node_id: usize, unit: Unit) {
        if node_id < self.node_units.len() && unit.id < self.unit_to_node.len() {
            if self.unit_to_node[unit.id].is_some() {
                panic!("Unit {} already assigned to a node", unit.id);
            }
            self.node_units[node_id].push(unit.clone());
            self.unit_to_node[unit.id] = Some(node_id);
        } else {
    
        }
    }

    /// 获取指定节点上的所有单元
    fn get_units_on_node(&self, node_id: usize) -> Option<&Vec<Unit>> {
        self.node_units.get(node_id)
    }

    /// 获取指定单元所在的节点
    fn get_node_of_unit(&self, unit_id: usize) -> Option<usize> {
        self.unit_to_node.get(unit_id).and_then(|opt| *opt)
    }

    /// 获取指定节点的邻居节点及其边数据
    fn get_nearest_nodes(&self, node_id: usize) -> Option<&Vec<(usize, EdgeData)>> {
        self.adjacency_list.get(node_id)
    }
}

#[no_mangle]
pub extern "C" fn create_graph() -> *mut Graph {
    unsafe {
        rust_log_printf(b"rust create_graph\n\0".as_ptr() as *const c_char);
    }
    let num_nodes = 10;
    let max_unit_id = 100;
    let graph = Graph::new(num_nodes, max_unit_id);
    let boxed_graph = Box::new(graph);
    Box::into_raw(boxed_graph)
}

#[no_mangle]
pub extern "C" fn destroy_graph(graph: *mut Graph) {
    unsafe {
        drop(Box::from_raw(graph));
    }
}

#[no_mangle]
pub extern "C" fn add_edge(graph: *mut Graph, from: usize, to: usize, weight: f32) {
    if graph.is_null() {
        return;
    }
    let edge_data = EdgeData { weight };
    unsafe {
        (*graph).add_edge(from, to, edge_data);
    }
}
