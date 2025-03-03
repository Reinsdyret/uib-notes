total_time_faults = 0
total_capacity_faults = 0
total_subroute_faults = 0
time_fault = 'Vehicle did not have time for pickup'
capacity_fault = 'Vehicle ran out of capacity'
subroute_fault = 'No subroutes found'
with open('output', 'r') as f:
    for line in f.readlines():
        if time_fault in line: total_time_faults += 1
        if capacity_fault in line: total_capacity_faults += 1
        if subroute_fault in line: total_subroute_faults += 1

print(f"Total time faults: {total_time_faults}")
print(f"Total capacity faults: {total_capacity_faults}")
print(f"total subroute faults: {total_subroute_faults}")
print(f"Total faults: {total_capacity_faults + total_time_faults}")
