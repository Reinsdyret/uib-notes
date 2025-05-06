// Function prototypes for MIS algorithms
void smis(int n, int *ver, int *edges, int *p, int *p1, int *p2, int *p3);
void pmis(int n, int *ver, int *edges, int *p, int *p2, int *p3, int *p4);
void lubys(int n, int *ver, int *edges, int *p, int *p2, int *p3, int *p4);

// Include C files after the prototypes to avoid redefinition issues
#include "mmio.c"       // Matrix market routines for reading files
// #include "numa.h"

#include "smis.c"    // Sequential maximal independent set algorithm
#include "pmis.c"    // Parallel maximal independent set algorithm
#include "lubys.c"   // Luby's parallel maximal independent set algorithm

