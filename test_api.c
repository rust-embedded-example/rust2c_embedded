/*
 * Simple test for the new linked list-based unit API
 */

#include "Grap.h"
#include <stdio.h>
#include <assert.h>

int main() {
    printf("Testing linked list-based unit API...\n");
    
    // Create a graph
    struct GraphHandle* graph = create_graph();
    assert(graph != NULL);
    
    // Test 1: Empty node should return NULL
    struct UnitData* empty_list = graph_get_first_unit(graph, 0);
    assert(empty_list == NULL);
    printf("✓ Empty node returns NULL\n");
    
    // Test 2: Add single unit
    struct UnitData unit1 = {.value = 100, .next = NULL};
    intptr_t result = graph_add_unit(graph, 0, unit1);
    assert(result >= 0);
    
    struct UnitData* first = graph_get_first_unit(graph, 0);
    assert(first != NULL);
    assert(first->value == 100);
    assert(first->next == NULL);
    printf("✓ Single unit added and retrieved correctly\n");
    
    // Test 3: Add multiple units
    struct UnitData unit2 = {.value = 200, .next = NULL};
    struct UnitData unit3 = {.value = 300, .next = NULL};
    
    graph_add_unit(graph, 0, unit2);
    graph_add_unit(graph, 0, unit3);
    
    // Verify linked list
    first = graph_get_first_unit(graph, 0);
    assert(first != NULL);
    assert(first->value == 100);
    
    struct UnitData* second = first->next;
    assert(second != NULL);
    assert(second->value == 200);
    
    struct UnitData* third = second->next;
    assert(third != NULL);
    assert(third->value == 300);
    assert(third->next == NULL);
    
    printf("✓ Multiple units linked correctly\n");
    
    // Test 4: Different nodes have separate lists
    struct UnitData unit4 = {.value = 400, .next = NULL};
    graph_add_unit(graph, 1, unit4);
    
    struct UnitData* node1_first = graph_get_first_unit(graph, 1);
    assert(node1_first != NULL);
    assert(node1_first->value == 400);
    assert(node1_first->next == NULL);
    
    // Node 0 should still have its list intact
    first = graph_get_first_unit(graph, 0);
    assert(first != NULL);
    assert(first->value == 100);
    
    printf("✓ Different nodes have separate lists\n");
    
    // Clean up
    graph_free_unit_list(graph_get_first_unit(graph, 0));
    graph_free_unit_list(graph_get_first_unit(graph, 1));
    destroy_graph(graph);
    
    printf("✓ All tests passed!\n");
    return 0;
}
