

// Parallel Breadth First Search
// -----------------------------
// Performs a BFS starting from vertex 1
// The parent of each vertex in the BFS tree along with its distance from the starting
// vertex is computed.
//
// The algorithm should gather all discovered vertices from round i, so that they can be 
// distributed evenly among the threads before the search in round i+1.
//
// Parameters:
// n     : number of vertices
// ver   : array of length n. ver[i] points to the start of the neighbor list of vertex i in edges
// edges : array containing lists of neighbors for each vertex, each edge is listed in both direction
// p     : array of length n used for parent pointers
// dist  : array of length n used for distance from starting vertex
// S     : array of length n used for maintaining queue of vertices to be processed 
// T     : array of length n where n >> number of threads.
//
// Note that the vertices are numbered from 1 to n (inclusive). Thus there is
// no vertex 0.

void pbfs(int n,int *ver,int *edges,int *p,int *dist,int *S,int *T) {
  int i,j;
  int v,w;          // Pointers to vertices
  int num_r;         // Number of vertices in S
  
  int threads = omp_get_num_threads();
  int thread_id = omp_get_thread_num();

  int *found = (int *)malloc(n * sizeof(int));
  int num_found = 0;


  #pragma omp for
  for(i = 1; i <= n; i++) {   // Set that every node is unvisited
    p[i] = -1;          // Using -1 to mark that a vertex is unvisited
    dist[i] = -1;
  }

  p[1] = 1;        // Set the parent of starting vertex to itself
  dist[1] = 0;     // Set the distance from the starting vertex to itself
  S[0] = 1;        // Add the starting vertex to S

  num_r = 1;       // Number of vertices in S

  int dist_next = 0;

  while (num_r != 0) {
    dist_next++;
    num_found = 0;

    #pragma omp for
    for(i = 0; i < num_r ; i++) {    // Loop over vertices in S
      v = S[i];                      // Grab next vertex v in S

      for(j = ver[v]; j < ver[v+1]; j++) { // Go through the neighbors of v
        w = edges[j];                // Get next neighbor w of v

        if (p[w] == -1) {            // Check if w is undiscovered
          p[w] = v;                  // Set v as the parent of w
          dist[w] = dist_next;       // Set distance of w 
          found[num_found++] = w;
        }
      } 
    }  

    // Each thread knows how many it found
    // Make that count public
    T[thread_id] = num_found;
    #pragma omp barrier

    // Do prefix sum over to find the offset for each thread to use in S
    num_r = T[0];
    int thread_offset = 0;
    for (int i = 1; i < threads; i++) {
      // Find offset for this thread
      if (i == thread_id) {
        thread_offset = num_r;
      }
      num_r += T[i];
    }

    // Place the found neighbours into S
    if (num_found > 0) {
      for (int i = 0; i < num_found; i++) {
        S[thread_offset + i] = found[i];
      }
    }

    #pragma omp barrier
  } 

}
