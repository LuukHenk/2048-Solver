# Twenty-Forty-Eight **Remake**

## Authors:
- [Luuk Perdaems](https://github.com/LuukHenk)
- [Sidney Liebrand](https://github.com/SidOfc)

## Status
Version: 0.0.3

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
    - Finds the the highest value on the board, determines if it is in a corner, and determines with which movement it gets in/out of the corner
- ~~The game can be played on an extern graphical screen~~

## Changelog:
    - Reorganised the determine_best_movement function in the lib/network.py file and added printing of the weights

## To do:
- Rebuild the AI that solved the game
    - Use the amount of empty tiles after movement as reference
    - ~~Use the score change after movement as reference~~
    - Find the the highest value on the board
        - ~~determines if the highest value is in a corner, and determines with which movement the highest value gets in/out of the corner~~
        - determines if the highest value is on the side, and determines with which movement the highest value gets to or from the side
- Build a extern graphical screen to play the game or watch the game being played
- Add timer
- Add highscore
    - Add log file
    - Add highscores.json
- Add the usage of user arguments
- Add multithreading
- Rewrite code in rust? ^^


