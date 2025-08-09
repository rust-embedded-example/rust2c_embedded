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

    let boxed = Box::new(GraphHandle {
        inner: g,
    });
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
    g.add_edge(from, to, EdgeData::new_with_id(eid_u32)).is_ok()
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

    let mut new_unit_data = unit_data;
    new_unit_data.next = core::ptr::null_mut();

    // 先添加新单元到图中
    let new_unit_id = match graph_handle.inner.add_unit_on_node(node_id, new_unit_data) {
        Ok(unit_id) => unit_id,
        Err(_) => return -1
    };

    // 获取新添加单元的地址，用于设置旧单元的指针
    let new_unit_ptr = if let Some(new_unit) = graph_handle.inner.unit(new_unit_id) {
        &new_unit.data as *const UnitData as *mut UnitData
    } else {
        return -1;
    };

    // 如果之前有单元，让最后一个旧单元指向新单元
    if let Some(existing_unit_ids) = graph_handle.inner.units_on_node(node_id) {
        if existing_unit_ids.len() > 1 {  // 新单元已添加，所以 > 1 表示有旧单元
            // 找到新添加之前的最后一个单元（即倒数第二个）
            let last_old_unit_id = existing_unit_ids[existing_unit_ids.len() - 2];
            if let Some(last_old_unit) = graph_handle.inner.unit_mut(last_old_unit_id) {
                // 让旧单元指向新单元
                last_old_unit.data.next = new_unit_ptr;
                info!("old unit {} now points to new unit {}", last_old_unit_id, new_unit_id);
            }
        }
    }

    new_unit_id as isize
}
#[unsafe(no_mangle)]
pub extern "C" fn graph_get_first_unit(graph: *const GraphHandle, node_id: usize) -> *mut UnitData {
    if graph.is_null() { return core::ptr::null_mut(); }
    let graph_handle = unsafe { &*graph };

    // Get units from the underlying graph
    if let Some(unit_ids) = graph_handle.inner.units_on_node(node_id) {
        if unit_ids.is_empty() {
            return core::ptr::null_mut();
        }

        if let Some(unit) = graph_handle.inner.unit(0) {
            // Return a pointer to the unit data in the underlying graph
            return &unit.data as *const UnitData as *mut UnitData;
        }
    }

    core::ptr::null_mut()
}

#[unsafe(no_mangle)]
pub extern "C" fn graph_visualize(graph: *const GraphHandle) {
    if graph.is_null() { return; }
    let g = unsafe { &(*graph).inner };
    g.log_adjacency();
}
