void smis(int n, int *ver, int *edges, int *p, int *p1, int *p2, int *p3) {
  // p1 will be used to track which vertices are still in V (1 = in V, 0 = not in V)
  // p2 will be used as our queue
  // p3 will be used to track queue indices
  
  // Initialize all vertices: not in p and all in V
  for (int i = 1; i <= n; i++) {
    p[i] = 0;
    p1[i] = 1;
  }
  
  // Initialize queue with all vertices
  int qfront = 0;
  int qback = 0;
  
  // Add all vertices to the queue
  for (int i = 1; i <= n; i++) {
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
      // Add v to the independent set p
      p[v] = 1;
      
      // Remove all neighbors of v from V
      for (int j = ver[v]; j < ver[v+1]; j++) {
          int neighbor = edges[j];
          p1[neighbor] = 0;
      }
    }
    
    // Remove v from V
    p1[v] = 0;
  }
}