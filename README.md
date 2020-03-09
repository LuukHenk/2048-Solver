# Twenty-Forty-Eight **Remake**
2048 is a simple (4x4) number board game in which you can merge equal values to the sum of the values using [w, s, a, d]. The goal of the game is to obtain a value of 2048. More info about the game [here](https://en.wikipedia.org/wiki/2048_(video_game)).

This program lets you play the 2048 game in the terminal using manual mode, or let the build in algorithm solve the game for you in automatic mode.


## Authors:
- [Luuk Perdaems](https://github.com/LuukHenk)
- [Sidney Liebrand](https://github.com/SidOfc)


## Status
Version: 0.0.3


## Quick start
### Requirements
    - argparse (verified for v3.2)
    - termios
    - fcntl
All requirements can be installed via pip3

### Installing:
Clone repository using git and run the main script to play the game:
```
$ git clone https://github.com/LuukHenk/2048-Solver.git
$ cd 2048-Solver
$ chmod +x main
```


## Usage:
Run `$ ./main manual` to manually play 2048 in the terminal.
Run `$ ./main automatic` to let the algorithm play 2048.

### Options:
`./main` also supports some options:
- `--total_games`: Determine the total games that will be played.
- `--render`: Render game in the terminal when running the automatic game (default: False).


## Changelog:
    - Reorganised the determine_best_movement function in the lib/network.py file and added the possibility to print the weights
    - Fixed determination if a game has been won
    - Reorganised main file by moving a lot of code to the lib/game_tools.py file
    - Added arguments
    - Reorganised README


## To do:
- Rebuild the AI that solved the game
    - ~~Use the score change after movement as reference~~
    - ~~determine if the highest value is in a corner, and determine with which movement the highest value gets in/out of the corner~~
    - determine how many high values are on the side and in the middle, and determines with which movement the highest values get to or from the side. Or even better, determine if the values are in consecutively order.
    - Use the amount of empty tiles after movement as reference

- Build a extern graphical screen to play the game or watch the game being played

- Add timer

- Add highscore
    - Add log file
    - Add highscores.json

- Add multithreading

- Add map structure to README

- Rewrite code in rust? ^^


