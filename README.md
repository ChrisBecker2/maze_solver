# maze_solver
Rust based breadth first search maze image solver

Works on black and white images with provided start and end points. 

Usage:
    maze_solver input.png start_x,start_y end_x,end_y output.png 

Examples:

    > maze_solver imgs/ratherlarge.png 2152,0 7276,7732 results/ratherlarge_result.png

        loading image: 0.402
        flood: 0.396
        draw_solution: 0.021
        save as png: 0.947

    maze_solver imgs/simple.png 290,0 400,581 results/simple_result.png 
    maze_solver imgs/circular.png 305,0 300,300 results/circular_result.png
    maze_solver imgs/distorted.png 0,46 787,761 results/distorted_result.png
    maze_solver imgs/hand_drawn.jpg 610,99 610,834 results/hand_drawn_result.png 
    maze_solver imgs/hand_drawn2.jpg 70,807 635,58 results/hand_drawn2_result.png

