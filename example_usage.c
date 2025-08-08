/*
 * Example usage of the new linked list-based unit API
 * 
 * This demonstrates how to use the new graph_get_first_unit() function
 * and traverse units using the 'next' field instead of the old iterator API.
 */

#include "Grap.h"  // Generated header file
#include <stdio.h>
#include <stddef.h>

void example_unit_traversal() {
    // Create a graph
    GraphHandle* graph = create_graph();
    if (!graph) {
        printf("Failed to create graph\n");
        return;
    }
    
    // Add some units to node 0
    struct UnitData unit1 = {.value = 100, .next = NULL};
    struct UnitData unit2 = {.value = 200, .next = NULL};
    struct UnitData unit3 = {.value = 300, .next = NULL};
    
    printf("Adding units to node 0...\n");
    graph_add_unit(graph, 0, unit1);
    graph_add_unit(graph, 0, unit2);
    graph_add_unit(graph, 0, unit3);
    
    // Get the first unit of node 0
    struct UnitData* first_unit = graph_get_first_unit(graph, 0);
    if (!first_unit) {
        printf("No units found on node 0\n");
        destroy_graph(graph);
        return;
    }

    // Traverse the linked list of units
    printf("Units on node 0:\n");
    struct UnitData* current = first_unit;
    int count = 0;

    while (current != NULL) {
        printf("  Unit %d: value = %u\n", count++, current->value);
        current = current->next;
    }

    // Clean up the unit list (important!)
    graph_free_unit_list(first_unit);
    
    // Test with another node
    printf("\nAdding units to node 1...\n");
    struct UnitData unit4 = {.value = 400, .next = NULL};
    struct UnitData unit5 = {.value = 500, .next = NULL};

    graph_add_unit(graph, 1, unit4);
    graph_add_unit(graph, 1, unit5);

    // Get units from node 1
    struct UnitData* first_unit_node1 = graph_get_first_unit(graph, 1);
    if (first_unit_node1) {
        printf("Units on node 1:\n");
        current = first_unit_node1;
        count = 0;

        while (current != NULL) {
            printf("  Unit %d: value = %u\n", count++, current->value);
            current = current->next;
        }

        // Clean up the unit list for node 1
        graph_free_unit_list(first_unit_node1);
    }

    // Clean up the graph
    destroy_graph(graph);
}

int main() {
    printf("Testing new linked list-based unit API\n");
    printf("======================================\n");
    example_unit_traversal();
    return 0;
}
