#!/usr/bin/env python3

with open('input.txt') as file:
    lines = file.readlines()

inventories = []

current_sum = 0
for line in lines:
    line = line.strip()
    if len(line) > 0:
        current_sum += int(line)
    else:
        inventories.append(current_sum)
        current_sum = 0


# part 1
print(max(inventories))

# part 2
print(sum(sorted(inventories)[-3:]))
