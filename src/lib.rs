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
use data::{UnitData, EdgeData};


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

    info!("create graph");

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
pub extern "C" fn add_edge(graph: *mut GraphHandle, from: usize, to: usize) -> bool {
    if graph.is_null() { return false; }
    let g = unsafe { &mut (*graph).inner };
    let eid_u32 = g.edge_count() as u32; // ensure EdgeData id == edge index
    g.add_edge(from, to, EdgeData::new_with_id(eid_u32)).is_some()
}

#[unsafe(no_mangle)]
pub extern "C" fn graph_neighbor_get(
    graph: *const GraphHandle,
    node_id: usize,
    index: usize,
    out_neighbor_id: *mut usize,
) -> *const EdgeData {
    if graph.is_null() || out_neighbor_id.is_null() { return core::ptr::null(); }
    let g = unsafe { &(*graph).inner };
    if let Some(neigh) = g.neighbors(node_id) {
        if index < neigh.len() {
            let (nbr, edge_id) = neigh[index];
            if let Some(edge) = g.edge(edge_id) {
                unsafe { *out_neighbor_id = nbr; }
                return &edge.data as *const EdgeData;
            }
        }
    }
    core::ptr::null()
}

#[unsafe(no_mangle)]
pub extern "C" fn graph_add_unit(graph: *mut GraphHandle, node_id: usize, unit_data: UnitData) -> isize {
    if graph.is_null() { return -1; }
    let g = unsafe { &mut (*graph).inner };
    match g.add_unit_on_node(node_id, unit_data) { Some(id) => id as isize, None => -1 }
}


pub struct UnitIterator {
    graph: *const GraphHandle,
    node_id: usize,
    unit: UnitData,
    current_index: usize,
}

#[unsafe(no_mangle)]
pub extern "C" fn graph_get_units_list(graph: *const GraphHandle, node_id: usize) -> *mut UnitIterator {
    if graph.is_null() { return core::ptr::null_mut(); }
    let g = unsafe { &(*graph).inner };

    // Check if node has any units
    if let Some(uids) = g.units_on_node(node_id) {
        if uids.is_empty() {
            return core::ptr::null_mut(); // No units, return NULL
        }

        // Get the first unit (index 0) from the node
        let first_unit = if let Some(unit) = g.unit(uids[0]) {
            unit.data
        } else {
            return core::ptr::null_mut(); // Invalid unit, return NULL
        };

        let iter = Box::new(UnitIterator {
            graph,
            node_id,
            unit: first_unit,
            current_index: 0,
        });
        Box::into_raw(iter)
    } else {
        core::ptr::null_mut() // Node not found or no units, return NULL
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn graph_units_next(iter: *mut UnitIterator) -> *mut UnitIterator {
    if iter.is_null() { return core::ptr::null_mut(); }
    let iterator = unsafe { &mut *iter };
    let g = unsafe { &(*iterator.graph).inner };

    // Move to next index
    iterator.current_index += 1;

    if let Some(uids) = g.units_on_node(iterator.node_id) {
        if iterator.current_index < uids.len() {
            let uid = uids[iterator.current_index];
            if let Some(unit) = g.unit(uid) {
                iterator.unit = unit.data;
                return iter;
            }
        }
    }
    core::ptr::null_mut()
}

#[unsafe(no_mangle)]
pub extern "C" fn graph_units_get_data(iter: *mut UnitIterator) -> *mut UnitData {
    if iter.is_null() {
        return core::ptr::null_mut();
    }
    let iterator = unsafe { &mut *iter };
    &mut iterator.unit as *mut UnitData
}

#[unsafe(no_mangle)]
pub extern "C" fn graph_units_iter_destroy(iter: *mut UnitIterator) {
    if !iter.is_null() {
        unsafe { drop(Box::from_raw(iter)); }
    }
}


#[unsafe(no_mangle)]
pub extern "C" fn graph_visualize(graph: *const GraphHandle) {
    if graph.is_null() { return; }
    let g = unsafe { &(*graph).inner };
    g.log_adjacency();
}
