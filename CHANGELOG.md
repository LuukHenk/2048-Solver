### Version 0.0.2 (Release: 7 March 2020)
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

### Version 0.0.1 (28 Februari 2020)
Made a playable 2048 game

- Remade the main file
- Reorganised the game class
- Removed some small bugs in the game class
- Finished the manual game, which is playable in the terminal
- Added comments to all code
- Made a new README.md


