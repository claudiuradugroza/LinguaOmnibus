/* io_kernels.c — File I/O helpers for std.io
 *
 * Compile: cc -O2 -shared -o libio.so io_kernels.c  (Linux)
 *         cl /O2 /LD io_kernels.c                   (Windows)
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* Read entire file into a malloc'd buffer. Caller must free().
   Returns NULL on error. Sets *out_len to file size. */
char* read_file_all(const char* path, int* out_len) {
    FILE* f = fopen(path, "rb");
    if (!f) return NULL;
    fseek(f, 0, SEEK_END);
    long sz = ftell(f);
    fseek(f, 0, SEEK_SET);
    if (sz < 0) { fclose(f); return NULL; }
    char* buf = (char*)malloc(sz + 1);
    if (!buf) { fclose(f); return NULL; }
    size_t rd = fread(buf, 1, sz, f);
    buf[rd] = '\0';
    fclose(f);
    if (out_len) *out_len = (int)rd;
    return buf;
}

/* Write a string to file. Returns 0 on success, -1 on error. */
int write_file_all(const char* path, const char* content) {
    FILE* f = fopen(path, "wb");
    if (!f) return -1;
    size_t len = strlen(content);
    size_t wr = fwrite(content, 1, len, f);
    fclose(f);
    return (wr == len) ? 0 : -1;
}

/* Get file size. Returns -1 on error. */
int file_size(const char* path) {
    FILE* f = fopen(path, "rb");
    if (!f) return -1;
    fseek(f, 0, SEEK_END);
    long sz = ftell(f);
    fclose(f);
    return (int)sz;
}