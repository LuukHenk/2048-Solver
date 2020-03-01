# Twenty-Forty-Eight **Remake**

## Authors:
- Luuk Perdaems
- [Sidney Liebrand](https://github.com/SidOfc)

## Status
Version: 0.0.1

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
- The game can be played automatically using a build-in algorithm (Not doing much at the moment)
- ~~The game can be played on an extern graphical screen~~


## Changelog:
### Version 0.0.1 (27 februari 2020)
- Remade the main file
- Reorganised the game class
- Removed some small bugs in the game class
- Finished the manual game, which is playable in the terminal
- Added comments to all code
- Made a new README.md

### Version 0.0.2 (...)
- Replaced the possible directions function which is now also used as the game_over function
- Simplified the perform_movement function in lib/game.py
- Connected 2048 algorithm
    - Use the score change for the next movement as reference by making a movement
- Bugfix in the lib/game.py comparing funcion

## To do:
- ~~Organise code of files in the lib folder~~
- Rebuild the AI that solved the game
    - Use the amount of empty tiles after movement as reference
    - ~~Use the score change after movement as reference~~
    - The more higher numbers in a single corner, or at least at one side, the better

- Build a extern graphical screen to play the game or watch the game being played
- Add highscore
