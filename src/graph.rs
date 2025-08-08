//! 图结构与相关操作的实现。
//!
//! 提供 `Graph`、`Unit`、`EdgeData` 及其基础操作。

use alloc::{vec, vec::Vec};
use log::{debug, error, info};
use alloc::string::String;
use alloc::format;

/// 表示图中可附着在节点上的“单元”。
#[repr(C)]
#[derive(Clone)]
pub struct Unit {
    /// 单元的唯一标识。
    pub id: u32,
}

/// 边数据。
#[repr(C)]
#[derive(Clone)]
pub struct EdgeData {
    /// 边的权重。
    pub weight: f32,
}

/// 无向图结构。
pub struct Graph {
    /// 邻接表：每个节点的邻居列表，包含邻居 ID 和边数据。
    pub adjacency_list: Vec<Vec<(usize, EdgeData)>>,
    /// 每个节点上的单元列表。
    pub node_units: Vec<Vec<Unit>>,
    /// 单元到节点的映射：索引是单元 ID，值是节点 ID。
    pub unit_to_node: Vec<Option<usize>>,
}

impl Graph {
    /// 创建一个新的图。
    ///
    /// - `num_nodes`：节点数量。
    /// - `max_unit_id`：允许的最大全元 ID（从 0 开始计数）。
    pub fn new(num_nodes: usize, max_unit_id: usize) -> Self {
        Graph {
            adjacency_list: vec![vec![]; num_nodes],
            node_units: vec![vec![]; num_nodes],
            unit_to_node: vec![None; max_unit_id + 1],
        }
    }

    /// 为无向图添加一条边（会同时添加 `from -> to` 与 `to -> from`）。
    pub fn add_edge(&mut self, from: usize, to: usize, edge_data: EdgeData) {
        debug!("add_edge: from {} to {}", from, to);
        if from < self.adjacency_list.len() && to < self.adjacency_list.len() {
            self.adjacency_list[from].push((to, edge_data.clone()));
            self.adjacency_list[to].push((from, edge_data));
        } else {
            error!(
                "Node index out of bounds: from={}, to={}, max_nodes={}",
                from,
                to,
                self.adjacency_list.len()
            );
        }
    }

    /// 将单元添加到指定节点。
    ///
    /// 如果单元已在某个节点上，函数会 `panic!`。
    #[allow(dead_code)]
    pub fn add_unit_to_node(&mut self, node_id: usize, unit: Unit) {
        let uid = unit.id as usize;
        if node_id < self.node_units.len() && uid < self.unit_to_node.len() {
            if self.unit_to_node[uid].is_some() {
                panic!("Unit {} already assigned to a node", unit.id);
            }
            self.node_units[node_id].push(unit.clone());
            self.unit_to_node[uid] = Some(node_id);
        } else {
            error!(
                "Node or unit index out of bounds: node_id={}, unit_id={}, max_nodes={}, max_unit_id={}",
                node_id,
                unit.id,
                self.node_units.len(),
                self.unit_to_node.len()
            );
        }
    }

    /// 获取指定节点上的所有单元。
    #[allow(dead_code)]
    pub fn get_units_on_node(&self, node_id: usize) -> Option<&Vec<Unit>> {
        self.node_units.get(node_id)
    }

    /// 获取指定单元所在的节点。
    #[allow(dead_code)]
    pub fn get_node_of_unit(&self, unit_id: u32) -> Option<usize> {
        let uid = unit_id as usize;
        self.unit_to_node.get(uid).and_then(|opt| *opt)
    }

    /// 获取指定节点的邻居节点及其边数据。
    #[allow(dead_code)]
    pub fn get_nearest_nodes(&self, node_id: usize) -> Option<&Vec<(usize, EdgeData)>> {
        self.adjacency_list.get(node_id)
    }

    /// 将图结构可视化输出到日志终端。
    ///
    /// 首行换行，之后将图的结构显示输出。
    pub fn visualize(&self) {
        let n = self.adjacency_list.len();
        info!("+---------------- Adjacency Matrix ----------------+");

        // Header row
        let mut header = String::from("     ");
        for j in 0..n {
            header.push_str(&format!(" {:>3}", j));
        }
        info!("{}", header);

        // Rows
        for i in 0..n {
            let mut row: Vec<Option<i32>> = vec![None; n];
            for (nbr, edge) in &self.adjacency_list[i] {
                // 仅展示整数权重，避免嵌入式小数格式化开销
                row[*nbr] = Some(edge.weight as i32);
            }
            let mut line = format!("{:>3} |", i);
            for j in 0..n {
                match row[j] {
                    Some(w) => line.push_str(&format!(" {:>3}", w)),
                    None => line.push_str("   ."),
                }
            }
            info!("{}", line);
        }

        info!("Legend: '.' no edge, numbers = weight (int)");

        // Units on nodes (streamed, low-allocation)
        info!("Units on nodes:");
        for i in 0..n {
            info!("  Node {}", i);
            if let Some(units) = self.node_units.get(i) {
                if units.is_empty() {
                    info!("    []");
                } else {
                    info!("    Units:");
                    for u in units {
                        info!("      - {}", u.id);
                    }
                }
            } else {
                info!("    []");
            }
        }

        // Node unit count table
        info!("+---------------- Node Unit Count ----------------+");
        info!("| Node ID | Unit Count |");
        info!("+---------+------------+");
        for i in 0..n {
            if let Some(units) = self.node_units.get(i) {
                info!("| {:>7} | {:>10} |", i, units.len());
            }
        }
        info!("+---------+------------+");

        info!("+--------------------------------------------------");
    }
}
