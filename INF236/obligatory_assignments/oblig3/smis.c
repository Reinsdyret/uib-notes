// No include needed here as driver.h is already included by cFiles.h
// Stub implementation of sequential MIS: marks all vertices as in the set
// No include needed here as driver.h is already included by cFiles.h
// Stub implementation of sequential MIS: marks all vertices as in the set
void smis(int n, int *ver, int *edges, int *p, int *p1, int *p2, int *p3) {
  // p1 will be used to track which vertices are still in V (1 = in V, 0 = not in V)
    // p2 will be used as our queue
    // p3 will be used to track queue indices
    
    // Initialize all vertices: not in I and all in V
    for (int i = 1; i < n; i++) {
      p[i] = 0;       // Initially no vertices in independent set I
      p1[i] = 1;      // Initially all vertices in set V
  }
  
  // Initialize queue with all vertices
  int qfront = 0;     // Front of queue (for dequeue)
  int qback = 0;      // Back of queue (for enqueue)
  
  // Add all vertices to the queue
  for (int i = 1; i < n; i++) {
      p2[qback++] = i;
  }
  
  // While queue is not empty
  while (qfront < qback) {
    // Dequeue a vertex v
    int v = p2[qfront++];
    
    // Skip if already removed from V
    if (p1[v] == 0) continue;
    
    // Check if any neighbors are already in the MIS
    int has_neighbor_in_mis = 0;
    
    for (int j = ver[v]; j < ver[v+1]; j++) {
        int neighbor = edges[j];
        if (p[neighbor] == 1) {
            has_neighbor_in_mis = 1;
            break;
        }
    }
    
    // Only add v to MIS if no neighbors are in MIS
    if (!has_neighbor_in_mis) {
        // Add v to the independent set I
        p[v] = 1;
        
        // Remove all neighbors of v from V
        for (int j = ver[v]; j < ver[v+1]; j++) {
            int neighbor = edges[j];
            p1[neighbor] = 0;  // Remove from V
        }
    }
    
    // Remove v from V (whether or not it was added to MIS)
    p1[v] = 0;
}

for (int v = 1; v <= n; v++) {
  if (p[v] == 1) {
      int start = ver[v];
      int end = ver[v+1];
      for (int j = start; j < end; j++) {
          int neighbor = edges[j];
          // If both in MIS, fix by removing the larger-indexed one
          if (p[neighbor] == 1) {
            printf("Found one");
              if (v < neighbor) {
                  p[neighbor] = 0;
              } else {
                  p[v] = 0;
              }
          }
      }
  }
}
}
