// No include needed here as driver.h is already included by cFiles.h
// Stub implementation of parallel MIS: marks all vertices as in the set
void pmis(int n, int *ver, int *edges, int *p, int *p2, int *p3, int *p4) {
    for (int v = 1; v <= n; v++) {
        p[v] = 1;
    }
}
