#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
extern crate alloc;

use core::panic::PanicInfo;
use core::alloc::Layout;
use alloc::boxed::Box;

use freertos_rust::FreeRtosAllocator;
use log::{info, error};

use grap_rs as graph;
mod data;
use data::{UnitData, EdgeData, EdgeIdLike, U32Value};

// Opaque handle for C: do not expose generic type to C, only use pointer to this struct
pub struct GraphHandle {
    inner: graph::Graph<(), UnitData, EdgeData>,
}

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

#[unsafe(no_mangle)]
pub extern "C" fn create_graph() -> *mut GraphHandle {
    // init logger once
    ccu_log::init_with_level(log::LevelFilter::Debug);

    info!("create_graph");

    let mut g = graph::Graph::with_capacities(10, 100, 16);
    // Pre-create 10 nodes with no data
    for _ in 0..10 { let _ = g.add_node(()); }

    let boxed = Box::new(GraphHandle { inner: g });
    Box::into_raw(boxed)
}

#[unsafe(no_mangle)]
pub extern "C" fn destroy_graph(graph: *mut GraphHandle) {
    if graph.is_null() { return; }
    unsafe { drop(Box::from_raw(graph)); }
}

#[unsafe(no_mangle)]
pub extern "C" fn add_edge(graph: *mut GraphHandle, from: usize, to: usize, _weight: f32) -> bool {
    if graph.is_null() { return false; }
    let g = unsafe { &mut (*graph).inner };
    let eid_u32 = g.edge_count() as u32; // ensure EdgeData id == edge index
    g.add_edge(from, to, EdgeData::new_with_id(eid_u32)).is_some()
}

// --- Neighbor queries (directly connected nodes) ---
#[unsafe(no_mangle)]
pub extern "C" fn graph_neighbors_len(graph: *const GraphHandle, node_id: usize) -> usize {
    if graph.is_null() { return 0; }
    let g = unsafe { &(*graph).inner };
    g.neighbors(node_id).map(|v| v.len()).unwrap_or(0)
}

#[unsafe(no_mangle)]
pub extern "C" fn graph_neighbor_get(
    graph: *const GraphHandle,
    node_id: usize,
    index: usize,
    out_neighbor_id: *mut usize,
    out_weight: *mut f32,
) -> bool {
    if graph.is_null() || out_neighbor_id.is_null() || out_weight.is_null() { return false; }
    let g = unsafe { &(*graph).inner };
    if let Some(neigh) = g.neighbors(node_id) {
        if index < neigh.len() {
            let (nbr, edge_id) = neigh[index];
            if let Some(edge) = g.edge(edge_id) {
                unsafe {
                    *out_neighbor_id = nbr;
                    *out_weight = edge.data.id_value() as f32; // return id-like value as f32
                }
                return true;
            }
        }
    }
    false
}

// --- Units on node ---
#[unsafe(no_mangle)]
pub extern "C" fn graph_add_unit(graph: *mut GraphHandle, node_id: usize, value: u32) -> isize {
    if graph.is_null() { return -1; }
    let g = unsafe { &mut (*graph).inner };
    match g.add_unit_on_node(node_id, UnitData::new(value)) { Some(id) => id as isize, None => -1 }
}

#[unsafe(no_mangle)]
pub extern "C" fn graph_units_len(graph: *const GraphHandle, node_id: usize) -> usize {
    if graph.is_null() { return 0; }
    let g = unsafe { &(*graph).inner };
    g.units_on_node(node_id).map(|v| v.len()).unwrap_or(0)
}

#[unsafe(no_mangle)]
pub extern "C" fn graph_units_get(
    graph: *const GraphHandle,
    node_id: usize,
    index: usize,
    out_unit_id: *mut usize,
    out_value: *mut u32,
) -> bool {
    if graph.is_null() || out_unit_id.is_null() || out_value.is_null() { return false; }
    let g = unsafe { &(*graph).inner };
    if let Some(uids) = g.units_on_node(node_id) {
        if index < uids.len() {
            let uid = uids[index];
            if let Some(unit) = g.unit(uid) {
                unsafe {
                    *out_unit_id = uid;
                    *out_value = unit.data.value();
                }
                return true;
            }
        }
    }
    false
}

// --- Visualization (Graphviz DOT) ---
#[unsafe(no_mangle)]
pub extern "C" fn graph_log_dot(graph: *const GraphHandle) {
    if graph.is_null() { return; }
    let g = unsafe { &(*graph).inner };
    g.log_adjacency();
}
