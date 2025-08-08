#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

extern int rust_log_printf(const char *format, ...);

/**
 * 创建第一个任务的函数
 * 该任务每秒打印一次 "Task 1 is running"
 */
int32_t create_task_1(void);

/**
 * 创建第二个任务的函数
 * 该任务每 500ms 打印一次 "Task 2 is running"
 */
int32_t create_task_2(void);

/**
 * 启动 FreeRTOS 调度器的函数
 * 在创建所有任务后调用此函数
 */
void start_freertos_scheduler(void);
