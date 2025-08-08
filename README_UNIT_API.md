# 单元链表 API 使用说明

## 概述

本项目已经重新设计了单元(Unit)的管理方式，现在每个节点上的单元通过链表连接。每个 `UnitData` 结构体包含一个 `next` 字段，指向同一节点上的下一个单元。

## 主要变化

### 1. UnitData 结构体更新

```c
typedef struct UnitData {
    uint32_t value;
    struct UnitData* next;  // 指向下一个单元的指针
} UnitData;
```

### 2. 新的 API

#### `graph_add_unit()`
```c
isize graph_add_unit(GraphHandle* graph, usize node_id, UnitData unit_data);
```
- 向指定节点添加单元
- 自动维护该节点的单元链表
- 新单元会被添加到链表末尾

#### `graph_get_first_unit()`
```c
UnitData* graph_get_first_unit(const GraphHandle* graph, usize node_id);
```
- 获取指定节点的第一个单元
- 返回 NULL 如果节点没有单元
- 使用返回的单元的 `next` 字段遍历整个链表

### 3. 删除的 API

以下旧的迭代器 API 已被删除：
- `graph_get_units_list()`
- `graph_units_next()`
- `graph_units_get_data()`
- `graph_units_iter_destroy()`

## 使用示例

```c
#include "Grap.h"

void traverse_units(GraphHandle* graph, usize node_id) {
    // 获取第一个单元
    struct UnitData* current = graph_get_first_unit(graph, node_id);

    if (current == NULL) {
        printf("No units found on node %zu\n", node_id);
        return;
    }

    // 遍历链表
    int count = 0;
    while (current != NULL) {
        printf("Unit %d: value = %u\n", count++, current->value);
        current = current->next;  // 移动到下一个单元
    }

    // 重要：释放链表内存
    graph_free_unit_list(graph_get_first_unit(graph, node_id));
}

int main() {
    GraphHandle* graph = create_graph();
    
    // 添加单元到节点 0
    struct UnitData unit1 = {.value = 100, .next = NULL};
    struct UnitData unit2 = {.value = 200, .next = NULL};
    struct UnitData unit3 = {.value = 300, .next = NULL};
    
    graph_add_unit(graph, 0, unit1);
    graph_add_unit(graph, 0, unit2);
    graph_add_unit(graph, 0, unit3);
    
    // 遍历节点 0 的所有单元
    traverse_units(graph, 0);
    
    destroy_graph(graph);
    return 0;
}
```

### 3. 新增的 API

#### `graph_free_unit_list()`
```c
void graph_free_unit_list(UnitData* first_unit);
```
- 释放从指定单元开始的整个链表
- **重要**: 使用完单元链表后必须调用此函数释放内存

## 重要说明

1. **内存管理**: 单元链表的内存需要手动管理，使用完毕后必须调用 `graph_free_unit_list()` 释放
2. **链表结构**: 每个节点的单元形成独立的链表，不同节点之间的单元不会相互连接
3. **添加顺序**: 新添加的单元会被链接到现有链表的末尾
4. **线程安全**: API 不是线程安全的，需要外部同步
5. **双重存储**: 单元数据既存储在底层图库中，也存储在我们的链表中

## 迁移指南

如果你之前使用旧的迭代器 API，请按以下方式迁移：

### 旧代码:
```c
UnitIterator* iter = graph_get_units_list(graph, node_id);
while (iter != NULL) {
    UnitData* data = graph_units_get_data(iter);
    printf("Value: %u\n", data->value);
    iter = graph_units_next(iter);
}
graph_units_iter_destroy(iter);
```

### 新代码:
```c
struct UnitData* first_unit = graph_get_first_unit(graph, node_id);
struct UnitData* current = first_unit;
while (current != NULL) {
    printf("Value: %u\n", current->value);
    current = current->next;
}
// 重要：释放链表内存
graph_free_unit_list(first_unit);
```
