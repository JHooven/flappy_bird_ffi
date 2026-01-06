#include <stdio.h>
#include <stdlib.h>

// Function that takes a void* pointer (e.g., generic data handler)
void handle_data(void *ptr) {
    if (ptr != NULL) {
        int *int_ptr = (int *)ptr; // Cast back to correct type
        printf("Handling data: %d\n", *int_ptr);
    } else {
        printf("Received NULL pointer.\n");
    }
}

// Function that returns a void* (e.g., dynamically allocated buffer)
void *create_buffer() {
    int *buffer = (int *)malloc(sizeof(int));
    if (buffer != NULL) {
        *buffer = 42; // Store some value
        printf("Buffer created at %p with value %d\n", buffer, *buffer);
    }
    return (void *)buffer; // Return as void*
}

// Function that takes nothing and returns void (e.g., a reset function)
void reset_system() {
    printf("System reset!\n");
}

void free_memory(void *ptr) {
    if (ptr != NULL) {
        printf("C: Freeing memory at %p\n", ptr);
        free(ptr);
    }
}

const char* get_ascii_string() {
    return "Hello, C!";
}

void print_message(const char *msg) {
    if (msg != NULL) {
        printf("C received: %s\n", msg);
    } else {
        printf("Received NULL pointer!\n");
    }
}
