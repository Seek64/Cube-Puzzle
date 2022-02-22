This is a solver for a logic puzzle.
The puzzle consists of 25 identical pieces of the following shape:

| X   | X   | X   | X   |
|-----|-----|-----|-----|
|     | X   |     |     |

The goal of the puzzle is to combine then to fill a 5x5x5 cube.

The algorithm uses backtracking to find a solution.
It iterates over the empty slots in the following way:
 - Left to right (x - WIDTH)
 - Back to front (y - DEPTH)
 - Bottom to top (z - HEIGHT)

For each empty slot, it tries to insert a piece n. \
To fill the next slot [x,y,z], the following 24 configurations are possible:
 - [x,y,z], [x+1,y,z], [x+2,y,z], [x+3,y,z], [x+1,y+1,z]
 - [x,y,z], [x+1,y,z], [x+2,y,z], [x+3,y,z], [x+2,y+1,z]
 - [x,y,z], [x+1,y,z], [x+2,y,z], [x+3,y,z], [x+1,y,z+1]
 - [x,y,z], [x+1,y,z], [x+2,y,z], [x+3,y,z], [x+2,y,z+1]
 - [x,y,z], [x-2,y+1,z], [x-1,y+1,z], [x,y+1,z], [x+1,y+1,z]
 - [x,y,z], [x-1,y+1,z], [x,y+1,z], [x+1,y+1,z], [x+2,y+1,z]
 - [x,y,z], [x,y+1,z], [x,y+2,z], [x,y+3,z], [x-1,y+1,z]
 - [x,y,z], [x,y+1,z], [x,y+2,z], [x,y+3,z], [x,y+1,z+1]
 - [x,y,z], [x,y+1,z], [x,y+2,z], [x,y+3,z], [x+1,y+1,z]
 - [x,y,z], [x,y+1,z], [x,y+2,z], [x,y+3,z], [x-1,y+2,z]
 - [x,y,z], [x,y+1,z], [x,y+2,z], [x,y+3,z], [x,y+2,z+1]
 - [x,y,z], [x,y+1,z], [x,y+2,z], [x,y+3,z], [x+1,y+2,z]
 - [x,y,z], [x,y,z+1], [x-2,y,z+1], [x-1,y,z+1], [x+1,y,z+1]
 - [x,y,z], [x,y,z+1], [x-1,y,z+1], [x+1,y,z+1], [x+2,y,z+1]
 - [x,y,z], [x,y,z+1], [x,y-2,z+1], [x,y-1,z+1], [x,y+1,z+1]
 - [x,y,z], [x,y,z+1], [x,y-1,z+1], [x,y+1,z+1], [x,y+2,z+1]
 - [x,y,z], [x,y,z+1], [x,y,z+2], [x,y,z+3], [x-1,y,z+1]
 - [x,y,z], [x,y,z+1], [x,y,z+2], [x,y,z+3], [x,y-1,z+1]
 - [x,y,z], [x,y,z+1], [x,y,z+2], [x,y,z+3], [x+1,y,z+1]
 - [x,y,z], [x,y,z+1], [x,y,z+2], [x,y,z+3], [x,y+1,z+1]
 - [x,y,z], [x,y,z+1], [x,y,z+2], [x,y,z+3], [x-1,y,z+2]
 - [x,y,z], [x,y,z+1], [x,y,z+2], [x,y,z+3], [x,y-1,z+2]
 - [x,y,z], [x,y,z+1], [x,y,z+2], [x,y,z+3], [x+1,y,z+2]
 - [x,y,z], [x,y,z+1], [x,y,z+2], [x,y,z+3], [x,y+1,z+2]

These configurations were derived manually. \
They represent the 24 different orientations of a block.

_Disclaimer:_ \
While the solution is relatively concise and straightforward, it is not very efficient. \
It took roughly an hour, before the first solutions were found.
