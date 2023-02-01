### Version 0.4.0 - 1 Februari 2023
- Rewrote the game engine in rust
	- Use of bitshifting to make playing games go a lot faster
- Added trainingsrounds to the game engine
	- After each trainings round, the top games are selected and the engine will try to improve these games
- UI changes
	- Added possibility to walk over moves in a game
	- Cleanup of code

### Version 0.3.0 - 11 Juni 2022
- Write a new design for the game
- Refactored the Python game engine
- Improve the gui rendering
	- Started using Qt widgets for the gui rendering

### Version 0.2.0 - 4 Februari 2021
- Added gui for automatic mode (lib/gui_rendering)
	- Gui build using tkinter
- Updated the terminal rendering
	- Renamed lib/rendering.py to lib/terminal_rendering.py
	- Rewrite of code in terminal rendering file
- Made it possible to resize the board with the "board_size" input argument
- Reorganized input arguments in general

### Version 0.1.0 - 4 March 2020
- Reworked the algorithm
- Added the possibility to print weights in the determine_best_movement function
- Added input arguments
- Reorganised and updated README

### Version 0.0.2 - 7 March 2020
- Added 'automation' game mode that uses an algorithm to solve the game
	- Algorithm uses weights to determine the best movements
		- Score increasement affects the weight
		- Position of values affects the weight
		- Empty tiles affects weights
- Bug fixes:
    - Bugfix in the lib/game.py comparing funcion
    - Bug fixed where the script crashes in manual gamemode if a non-supported key was pressed
    - Bug fix in the determine_best_movement function in lib/network.py
    - Other small bugfixes
    - game.possible_directions returns invalid directions which cause game.game_over to stay False

### Version 0.0.1 - 28 Februari 2020
Made a playable 2048 game

- Run the game in the terminal using a main file
- Added a README.md


