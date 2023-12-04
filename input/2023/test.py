schematic = open('day3.txt', 'r').read().splitlines()

import re

def get_around(line_num, match):
    start = match.start()
    end = match.end()
    if start: start -= 1
    if end < 139: end += 1
    around = ''
    if line_num: around += schematic[line_num-1][start:end]
    around += schematic[line_num][start:end]
    if line_num < 139: around += schematic[line_num+1][start:end]
    return around

part_numbers = []

part_num_coords = []

for line_num, line in enumerate(schematic): 
    for match in re.finditer(r'\d+', line):
        if re.search(r'[^\d.]', get_around(line_num, match)):
            part_numbers.append(int(match[0]))
            part_num_coords.append({
                'line': line_num,
                'start': match.start(),
                'end': match.end()
                })

print(sum(part_numbers))

coords_parts = {}

for i, part in enumerate(part_num_coords):
    for col in range(part['start'], part['end']):
        coords_parts[(part['line'], col)] = i

gears = []

for line_num, line in enumerate(schematic): 
    for match in re.finditer(r'\*', line):
        adjacent_parts = set()
        pos = match.start()
        for col in range(pos-1, pos+2):
            for row in range(line_num-1, line_num+2):
                adjacent_parts.add(coords_parts.get((row, col)))
                adjacent_parts.discard(None)
        if len(adjacent_parts) == 2: 
            parts = tuple(adjacent_parts)
            gears.append(parts)
            print(f"({line_num}, {pos}) = {part_numbers[parts[0]]*part_numbers[parts[1]]}")

print(sum(part_numbers[g1]*part_numbers[g2] for g1, g2 in gears))
                