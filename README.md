# maze_solver
Solve black and white maze images

Does a flood fill that writes the distance from a start point until the end point is reached. Then a solution is drawn from the end by looking for the adjacent pixel where the distance decreases.


Usage:
    maze_solver input.png start_x,start_y end_x,end_y output.png 

Examples:

    > maze_solver imgs/ratherlarge.png 2152,0 7276,7732 results/ratherlarge_result.png

        loading image: 0.402
        flood: 0.396
        draw_solution: 0.021
        save as png: 0.947

    maze_solver imgs/circu.png 2152,0 7276,7732 results/ratherlarge_result.png

Image attribution:
ratherlarge.png - https://freesvg.org/ratherlargemaze
circular.png - https://freesvg.org/circular-maze
