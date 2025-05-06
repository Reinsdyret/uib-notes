// No include needed here as driver.h is already included by cFiles.h
// Stub implementation of Luby's parallel MIS: marks all vertices as in the set
void lubys(int n, int *ver, int *edges, int *p, int *p2, int *p3, int *p4) {
    for (int v = 1; v <= n; v++) {
        p[v] = 1;
    }
}
