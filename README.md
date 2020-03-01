# Twenty-Forty-Eight **Remake**

## Authors:
- Luuk Perdaems
- [Sidney Liebrand](https://github.com/SidOfc)

## Status
Version: 0.0.2

## Installing:
Clone repository using git and run the main script to play the game:
```
$ git clone https://github.com/LuukHenk/2048-Solver.git
$ cd 2048-Solver
$ chmod +x main
$ ./main
```
Follow the instructions in the main file from here

## Possibilities:
- The game can be played manually in the terminal using manual mode (run ./main)
- The game can be played automatically using a build-in algorithm
    - Predicts score of the possible movements and adds weight to the movement with the highest score
- ~~The game can be played on an extern graphical screen~~

## Changelog:
### Version 0.0.1 (27 Februari 2020)
- Remade the main file
- Reorganised the game class
- Removed some small bugs in the game class
- Finished the manual game, which is playable in the terminal
- Added comments to all code
- Made a new README.md

### Version 0.0.2 (1 March 2020)
- Rebuild and simplification of the function "possible_directions" in lib/game.py
- Connected 2048 algorithm
    - Predicts score of the possible movements and adds weight to the movement with the highest score
- Bugfix in the lib/game.py comparing funcion
- Other small bugfixes

## To do:
- Rebuild the AI that solved the game
    - Use the amount of empty tiles after movement as reference
    - ~~Use the score change after movement as reference~~
    - The more higher numbers in a single corner, or at least at one side, the better
- Build a extern graphical screen to play the game or watch the game being played
- Add highscore
