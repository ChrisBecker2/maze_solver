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

    maze_solver imgs/circular.png 305,0 300,300 results/circular_result.png
    maze_solver imgs/distorted.png 0,46 787,761 results/distorted_result.png
