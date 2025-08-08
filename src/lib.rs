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
use graph::{Graph, EdgeData, Unit};




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



/// Create a new graph instance and return an opaque pointer for C.
///
/// Safety: The returned pointer must later be passed to `destroy_graph` to free
/// the allocation. Do not free it from C by other means.
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

/// Destroy a graph previously created by `create_graph`.
///
/// Safety: `graph` must be a valid pointer returned by `create_graph`, and must
/// not be used after this call.
#[no_mangle]
pub extern "C" fn destroy_graph(graph: *mut Graph) {
    unsafe {
        drop(Box::from_raw(graph));
    }
}

/// Add an undirected edge (from <-> to) with the given weight.
///
/// If `graph` is null this is a no-op.
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

/// Render the graph into the logging terminal as an adjacency matrix and unit list.
#[no_mangle]
pub extern "C" fn visualize_graph(graph: *mut Graph) {
    if graph.is_null() {
        return;
    }
    unsafe {
        (*graph).visualize();
    }
}
/// Add a `Unit` with the given `unit_id` to the node `node_id`.
#[no_mangle]
pub extern "C" fn graph_add_unit(graph: *mut Graph, node_id: usize, unit: Unit) {
    if graph.is_null() { return; }
    unsafe { (*graph).add_unit_to_node(node_id, unit); }
}

/// Return the number of `Unit`s attached to the given node.
#[no_mangle]
pub extern "C" fn graph_units_len(graph: *const Graph, node_id: usize) -> usize {
    if graph.is_null() { return 0; }
    let g = unsafe { &*graph };
    match g.get_units_on_node(node_id) { Some(v) => v.len(), None => 0 }
}

/// Get the `Unit` id at `index` on `node_id`.
///
/// On success writes the id into `out_unit_id` and returns `true`. Otherwise returns `false`.
#[no_mangle]
pub extern "C" fn graph_units_get(graph: *const Graph, node_id: usize, index: usize, out_unit_id: *mut u32) -> bool {
    if graph.is_null() || out_unit_id.is_null() { return false; }
    let g = unsafe { &*graph };
    if let Some(v) = g.get_units_on_node(node_id) {
        if index < v.len() {
            unsafe { *out_unit_id = v[index].id; }
            return true;
        }
    }
    false
}

/// Find the node that contains `unit_id`.
///
/// On success writes the node id into `out_node_id` and returns `true`.
#[no_mangle]
pub extern "C" fn graph_get_node_of_unit(graph: *const Graph, unit_id: u32, out_node_id: *mut usize) -> bool {
    if graph.is_null() || out_node_id.is_null() { return false; }
    let g = unsafe { &*graph };
    if let Some(node) = g.get_node_of_unit(unit_id) {
        unsafe { *out_node_id = node; }
        return true;
    }
    false
}

/// Return the number of neighbors of the given node.
#[no_mangle]
pub extern "C" fn graph_neighbors_len(graph: *const Graph, node_id: usize) -> usize {
    if graph.is_null() { return 0; }
    let g = unsafe { &*graph };
    match g.get_nearest_nodes(node_id) { Some(v) => v.len(), None => 0 }
}

/// Get the neighbor at `index` for `node_id`, along with the edge weight.
///
/// On success writes into `out_neighbor_id` and `out_weight` and returns `true`.
#[no_mangle]
pub extern "C" fn graph_neighbor_get(graph: *const Graph, node_id: usize, index: usize, out_neighbor_id: *mut usize, out_weight: *mut f32) -> bool {
    if graph.is_null() || out_neighbor_id.is_null() || out_weight.is_null() { return false; }
    let g = unsafe { &*graph };
    if let Some(v) = g.get_nearest_nodes(node_id) {
        if index < v.len() {
            let (nid, edge) = v[index].clone();
            unsafe {
                *out_neighbor_id = nid;
                *out_weight = edge.weight;
            }
            return true;
        }
    }
    false
}

/// Return the total number of nodes in the graph.
#[no_mangle]
pub extern "C" fn graph_num_nodes(graph: *const Graph) -> usize {
    if graph.is_null() { return 0; }
    let g = unsafe { &*graph };
    g.adjacency_list.len()
}
