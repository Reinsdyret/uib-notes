
// Parallel Breadth First Search
// -----------------------------
// Berforms a BFS starting from vertex 1
// The parent of each vertex in the BFS tree along with its distance from the starting
// vertex is computed.
//
// The algorithm should first perform some rounds of sequential BFS before starting a parallel
// execution. In the parallel part each thread should be allocated a part of the vertices from the
// last round of the sequential algorithm. Any discovered vertices in the parallel part should 
// first be stored with the thread that discovered them and only if the most heavily loaded 
// thread has more than k vertices more than the average load should the vertices be 
// gathered and divided among the threads. 
//
//
// Parameters:
// n     : number of vertices
// ver   : ver[i] points to the start of the neighbor list of vertex i in edges
// edges : lists of neighbors of each vertex, each edge is listed in both direction
// p     : array of length n used for parent pointers
// dist  : array of length n used for distance from starting vertex
// S     : array of length n used for maintaining queue of vertices to be processed, only used in the 
//         sequential part. 
// T     : array of length n where n >> number of threads. 
//
// Note that the vertices are numbered from 1 to n (inclusive). Thus there is
// no vertex 0.

void abfs(int n,int *ver,int *edges,int *p,int *dist,int *S,int *T) {
  int sequential_steps = 6; // NEEDS TO BE EVEN
  int dist_next = sequential_steps;
  int i,j;
  int v,w;          // Pointers to vertices
  int num_r;         // Number of vertices in S
  
  int threads = omp_get_num_threads();
  int thread_id = omp_get_thread_num();

  int *found = (int *)malloc(n * sizeof(int));
  int *new_found = (int *)malloc(n * sizeof(int));
  int num_found = 0;
  int num_new_found = 0;

  // Run the sequential to get inital vertices to search
  #pragma omp single
  {
  num_r = sbfs_with_steps(sequential_steps, n, ver, edges, p, dist, S, T);
  T[0] = num_r;
  }

  num_r = T[0];

  // Distribute vertices from S
  int vertices_per_thread = num_r / threads;
  int extra_vertices = num_r % threads;
  
  int my_count;
  int my_start;
  
  if (thread_id < extra_vertices) {
      // First 'extra_vertices' threads get one extra vertex
      my_count = vertices_per_thread + 1;
      my_start = thread_id * my_count;
  } else {
      my_count = vertices_per_thread;
      my_start = (extra_vertices * (vertices_per_thread + 1)) + 
                ((thread_id - extra_vertices) * vertices_per_thread);
  }
    
  // Copy my assigned vertices from S back to my found array
  num_found = 0; 
  for (int i = 0; i < my_count; i++) {
      found[num_found++] = S[my_start + i];
  }

  // Continue in parallel
  while (num_r != 0) {
    #pragma omp barrier
    dist_next++;
    num_new_found = 0;

    for(i = 0; i < num_found ; i++) {    // Loop over vertices in S
      v = found[i];                      // Grab next vertex v in S
      int next_dist = dist_next;

      for(j = ver[v]; j < ver[v+1]; j++) { // Go through the neighbors of v
        w = edges[j];                // Get next neighbor w of v
        

        if (p[w] == -1) {            // Check if w is undiscovered
          p[w] = v;                  // Set v as the parent of w
          dist[w] = next_dist;       // Set distance of w 
          new_found[num_new_found++] = w;
        }
      } 
    }  

    // Each thread knows how many it found
    // Make that count public
    T[thread_id] = num_new_found;
    #pragma omp barrier

    // Do prefix sum over to find the offset for each thread to use in S
    num_r = T[0];
    int thread_offset = 0;
    int max_found = 0;
    for (int i = 1; i < threads; i++) {
      // Find offset for this thread
      if (i == thread_id) {
        thread_offset = num_r;
      }
      if (T[i] > max_found) {
        max_found = T[i];
      }
      num_r += T[i];
    }

    // If the max is double of the average
    int collect_condition = max_found >= 1.5 * (num_r / threads);

    if (num_r > 0 && collect_condition) {
      // Gather into S
      for (int i = 0; i < num_new_found; i++) {
        S[thread_offset + i] = new_found[i];
      }

      #pragma omp barrier
    
      // Distribute vertices from S
      int vertices_per_thread = num_r / threads;
      int extra_vertices = num_r % threads;
      
      
      int my_count;
      int my_start;
      
      if (thread_id < extra_vertices) {
          // First 'extra_vertices' threads get one extra vertex
          my_count = vertices_per_thread + 1;
          my_start = thread_id * my_count;
      } else {
          my_count = vertices_per_thread;
          my_start = (extra_vertices * (vertices_per_thread + 1)) + 
                    ((thread_id - extra_vertices) * vertices_per_thread);
      }
        
      // Copy my assigned vertices from S back to my found array
      num_found = 0; 
      for (int i = 0; i < my_count; i++) {
          found[num_found++] = S[my_start + i];
      }
    
    } else {
      int *temp = found;
      found = new_found;
      new_found = temp;
      num_found = num_new_found;
      num_new_found = 0;
    }
  }

}

int sbfs_with_steps(int steps, int n,int *ver,int *edges,int *p,int *dist,int *S,int *T) {
  int i,j;          // Loop indices
  int v,w;          // Pointers to vertices
  int num_r, num_w;  // Number of vertices in S and T, respectively
  int *temp;        // Temporary pointer
  int step = 0;

  for(i=1;i<=n;i++) {   // Set that every node is unvisited
    p[i] = -1;          // Using -1 to mark that a vertex is unvisited
    dist[i] = -1;
  }

  p[1] = 1;        // Set the parent of starting vertex to itself
  dist[1] = 0;     // Set the distance from the starting vertex to itself
  S[0] = 1;        // Add the starting vertex to S

  num_r = 1;       // Number of vertices in S
  num_w = 0;       // Number of vertices in T

  while (num_r != 0) {               // Loop until all vertices have been discovered
    if (step >= steps) {
      return num_r;
    }
    for(i=0;i<num_r;i++) {           // Loop over vertices in S
      v = S[i];                      // Grab next vertex v in S
      for(j=ver[v];j<ver[v+1];j++) { // Go through the neighbors of v
        w = edges[j];                // Get next neighbor w of v
        if (p[w] == -1) {            // Check if w is undiscovered
          p[w] = v;                  // Set v as the parent of w
          dist[w] = dist[v]+1;       // Set distance of w 
          T[num_w++] = w;            // Add w to T and increase number of vertices discovered 
        }
      }  // End loop over neighbors of v
    }  // End loop of vertices in S
    temp = S;  // Swap S and T
    S = T;
    T = temp;
    num_r = num_w; // Set number of elements in S
    num_w = 0;     // Set T as empty
    step++;
  } //  End loop over entire graph
}
