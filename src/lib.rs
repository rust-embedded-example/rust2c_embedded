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
use alloc::collections::BTreeMap;


pub struct GraphHandle {
    inner: graph::Graph<(), UnitData, EdgeData>,
    // Map from node_id to the first unit in the linked list for that node
    first_units: BTreeMap<usize, *mut UnitData>,
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

    let boxed = Box::new(GraphHandle {
        inner: g,
        first_units: BTreeMap::new()
    });
    Box::into_raw(boxed)
}

#[unsafe(no_mangle)]
pub extern "C" fn destroy_graph(graph: *mut GraphHandle) {
    if graph.is_null() { return; }

    let graph_handle = unsafe { &mut *graph };

    // Clean up all unit linked lists
    for (_, first_unit_ptr) in graph_handle.first_units.iter() {
        if !first_unit_ptr.is_null() {
            // Free the entire linked list for this node
            let mut current = *first_unit_ptr;
            while !current.is_null() {
                let next = unsafe { (*current).next() };
                unsafe { drop(alloc::boxed::Box::from_raw(current)); }
                current = next;
            }
        }
    }

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
    let graph_handle = unsafe { &mut *graph };

    // Add the unit to the underlying graph
    match graph_handle.inner.add_unit_on_node(node_id, unit_data) {
        Some(unit_id) => {
            // Create a boxed copy of the unit data for our linked list
            let mut new_unit = unit_data;
            new_unit.next = core::ptr::null_mut();
            let boxed_unit = alloc::boxed::Box::new(new_unit);
            let unit_ptr = alloc::boxed::Box::into_raw(boxed_unit);

            // Update the linked list for this node
            if let Some(&first_unit_ptr) = graph_handle.first_units.get(&node_id) {
                // Node already has units, add to the end of the list
                let mut current = first_unit_ptr;
                unsafe {
                    while !(*current).next().is_null() {
                        current = (*current).next();
                    }
                    (*current).set_next(unit_ptr);
                }
            } else {
                // This is the first unit for this node
                graph_handle.first_units.insert(node_id, unit_ptr);
            }

            unit_id as isize
        }
        None => -1
    }
}

/// Get the first unit of a node. Returns null if no units exist.
/// Use the 'next' field of the returned UnitData to traverse the linked list.
#[unsafe(no_mangle)]
pub extern "C" fn graph_get_first_unit(graph: *const GraphHandle, node_id: usize) -> *mut UnitData {
    if graph.is_null() { return core::ptr::null_mut(); }
    let graph_handle = unsafe { &*graph };

    // Simply return the first unit pointer for this node
    graph_handle.first_units.get(&node_id).copied().unwrap_or(core::ptr::null_mut())
}


// UnitIterator is no longer needed - we'll use linked list traversal instead

/// Free the linked list of units starting from the given unit.
/// This should be called to clean up memory when done with the unit list.
#[unsafe(no_mangle)]
pub extern "C" fn graph_free_unit_list(first_unit: *mut UnitData) {
    if first_unit.is_null() { return; }

    let mut current = first_unit;
    while !current.is_null() {
        let next = unsafe { (*current).next() };
        unsafe { drop(alloc::boxed::Box::from_raw(current)); }
        current = next;
    }
}

// Old iterator functions removed - use linked list traversal instead


#[unsafe(no_mangle)]
pub extern "C" fn graph_visualize(graph: *const GraphHandle) {
    if graph.is_null() { return; }
    let g = unsafe { &(*graph).inner };
    g.log_adjacency();
}
