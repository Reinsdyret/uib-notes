#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <omp.h>
#include <time.h>


// Luby's Algorithm for finding Maximal Independent Set (MIS)
void lubys(int n, int *ver, int *edges, int *status, int *queue, int *rand_vals, int *p4) {
    // Initialize random values (done once)
    // Init states
    unsigned int seed = time(NULL) + omp_get_thread_num();
    #pragma omp for
    for(int v = 1; v<=n; v++) {
        rand_vals[v] = rand_r(&seed);
        queue[v] = 1;
        status[v] = 0;
    }

    int *local_mis = malloc((n + 1) * sizeof(int));
    int local_count = 0;

    int someone_in_queue = 1;

    while (someone_in_queue) {
        #pragma omp barrier

        // Phase 1: Find local MIS candidates
        int num_threads = omp_get_num_threads();
        int tid = omp_get_thread_num();
        local_count = 0;
        

        #pragma omp for schedule(static)
        for (int v = 1; v <= n; v++) {
            if (!queue[v]) continue;

            int dominates = 1;
            for (int j = ver[v]; j < ver[v + 1]; j++) {
                int w = edges[j];
                if (!queue[w]) continue;

                if (rand_vals[w] > rand_vals[v] || (rand_vals[w] == rand_vals[v] && w > v)) {
                    dominates = 0;
                    break;
                }
            }

            if (dominates) {
                local_mis[local_count++] = v;
            }
        }

        #pragma omp barrier


        for (int i = 0; i < local_count; i++) {
            int v = local_mis[i];
            if (!queue[v]) continue;

            queue[v] = 0;
            status[v] = 1;

            for (int j = ver[v]; j < ver[v + 1]; j++) {
                int w = edges[j];
                if (queue[w]) {
                    queue[w] = 0;
                    status[w] = 0;
                }
            }
        }

        #pragma omp barrier

        // Check for termination

        someone_in_queue = 0;
        p4[0] = 0;

        for (int v = 1; v <= n; v++) {
            if (queue[v]) {
                p4[0] = 1;
                break;
            }
        }

        someone_in_queue = p4[0];
    }

}
