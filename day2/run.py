#!/usr/bin/env python

with open("input.txt") as file:
    lines = file.readlines()

part1_score = 0
part2_score = 0
for line in lines:
    their_shape, my_shape = line.strip().split()
    their_shape = ord(their_shape) - ord('A')
    my_shape = ord(my_shape) - ord('X')

    shape_score = my_shape + 1

    round_score = {
        2: 0,  # they won
        1: 6,  # I won
        0: 3,  # draw
        -1: 0,  # they won
        -2: 6,  # I won
    }[my_shape - their_shape]
    part1_score += round_score + shape_score

    # 0: lose
    # 1: draw
    # 2: win
    round_score = my_shape * 3
    my_shape = (their_shape + my_shape - 1) % 3
    shape_score = my_shape + 1
    part2_score += round_score + shape_score

print('part 1', part1_score)
print('part 2', part2_score)
