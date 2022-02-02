# maze_solver
Solve black and white maze images

Does a flood fill that writes the distance from a start point until the end point is reached. Then a solution is drawn from the end by looking for the adjacent pixel where the distance decreases.


Examples:


    > maze_solver ratherlarge.png 2152,0 7276,7732 result.png

    loading image: 0.402
    flood: 0.396
    draw_solution: 0.021
    save as png: 0.947


Image attribution:
ratherlarge.png - Converted to PNG from https://freesvg.org/ratherlargemaze
