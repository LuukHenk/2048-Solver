### Version 0.3.0 - 11 Juni 2022
- Write a new design for the game
- Refactored the Python game engine
- Improve the HMI
	- Started using Qt widgets for the HMI

### Version 0.2.0 - 4 Februari 2021
- Added gui for automatic mode (lib/gui_rendering)
	- Gui build using tkinter
- Updated the terminal rendering
	- Renamed lib/rendering.py to lib/terminal_rendering.py
	- Rewrite of code in terminal rendering file
- Made it possible to resize the board with the "board_size" input argument
- Reorganized input arguments in general

### Version 0.1.0 - 4 March 2020
- Rename of lib/network.py to lib/algorithm.py
    - Reorganisation of lib/algorithm.py, with focus on the movement determination function
		- Added the possibility to print weights in the determine_best_movement function
- Fixed "game over" and "game won" code
- Moved rendering of the game to lib/rendering.py
- Added input arguments
- Reorganised and updated README

### Version 0.0.2 - 7 March 2020
Made a automated gameplay using a algorithm that uses the score and highest value on the board to determine the best movement

- Rebuild and simplification of the function "possible_directions" in lib/game.py
- Connected 2048 algorithm
    - Predicts score of the possible movements and adds weight to the movement with the highest score
    - Finds the the highest value on the board, determines if it is in a corner, and determines with which movement it gets in/out of the corner

- Bug fixes:
    - Bugfix in the lib/game.py comparing funcion
    - Bug fixed where the script crashes in manual gamemode if a non-supported key was pressed
    - Bug fix in the determine_best_movement function in lib/network.py
    - Other small bugfixes
    - game.possible_directions returns invalid directions which cause game.game_over to stay False

### Version 0.0.1 - 28 Februari 2020
Made a playable 2048 game

- Remade the main file
- Reorganised the game class
- Removed some small bugs in the game class
- Finished the manual game, which is playable in the terminal
- Added comments to all code
- Made a new README.md


