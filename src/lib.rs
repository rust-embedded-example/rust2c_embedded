#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
extern crate alloc;

use core::panic::PanicInfo;

use freertos_rust::FreeRtosAllocator;
use core::alloc::Layout;
use alloc::boxed::Box;

mod logging;
mod graph;

use log::{info, error};
use graph::{Graph, EdgeData};



#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    error!("alloc error");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    error!("panic");
    loop {}
}

#[global_allocator]
static GLOBAL: FreeRtosAllocator = FreeRtosAllocator;



#[no_mangle]
pub extern "C" fn create_graph() -> *mut Graph {
    logging::init().unwrap();

    // 使用 log crate 的不同级别日志，都会通过 sys_log 输出
    info!("create_graph");

    info!("graph: nodes={}, max_unit_id={}", 10, 100);

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

#[no_mangle]
pub extern "C" fn visualize_graph(graph: *mut Graph) {
    if graph.is_null() {
        return;
    }
    unsafe {
        (*graph).visualize();
    }
}