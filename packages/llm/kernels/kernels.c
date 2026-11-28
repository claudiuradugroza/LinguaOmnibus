/* kernels.c — reference C kernels for the llm package
 *
 * These are the heavy kernels that, in a real build, would be hand-written
 * in x86-64 assembly (see llm.asm). For now they are plain C so the
 * package can be built and benchmarked. Lingua Omnibus calls them through
 * the (future) FFI layer.
 *
 * Compile:  cc -O3 -march=native -c kernels.c -o kernels.o
 */

#include <math.h>

void rms_norm(float *x, const float *w, int n, float eps) {
    float sq = 0.0f;
    for (int i = 0; i < n; i++) sq += x[i] * x[i];
    float inv = 1.0f / sqrtf(sq / n + eps);
    for (int i = 0; i < n; i++) x[i] = x[i] * inv * w[i];
}

void softmax(float *x, int n) {
    float m = x[0];
    for (int i = 1; i < n; i++) if (x[i] > m) m = x[i];
    float s = 0.0f;
    for (int i = 0; i < n; i++) { x[i] = expf(x[i] - m); s += x[i]; }
    for (int i = 0; i < n; i++) x[i] /= s;
}

void matvec(const float *W, const float *x, float *y, int rows, int cols) {
    for (int r = 0; r < rows; r++) {
        float acc = 0.0f;
        for (int c = 0; c < cols; c++) acc += W[r * cols + c] * x[c];
        y[r] = acc;
    }
}