# Twenty-Forty-Eight - The Remake
#### Version 0.1.0
2048 is a simple numberbased board game with a 4x4 board. All the values on the board are two or a squired form of two. Values can be merged when equal by moving the values over the board using [w, s, a, d]. The goal of the game is to obtain a value of 2048. More info about the game [here](https://en.wikipedia.org/wiki/2048_(video_game)).

This program lets you play the 2048 game in the terminal ~~and in a gui~~ using manual mode. You can also let the build-in algorithm solve the game for you in automatic mode. This is an updated version of my very first programming project with [Sidney Liebrand](https://github.com/SidOfc).

## Quick start
### Requirements
* argparse (verified for v3.2)
* termios (only for manual mode)
* fcntl (only for manual mode)

All requirements can be installed via `$ pip3 install <requirement>`

### Installing:
The game can be installed by cloning this repository.
```
$ git clone https://github.com/LuukHenk/2048-Solver.git
$ cd 2048-Solver
$ chmod +x main
```


## Usage:
The game can be played manually or by the build in algorighm.
* Run `$ ./main manual` to manually play 2048 in the terminal.
* Run `$ ./main automatic` to let the build in (or your own) algorithm play 2048.

* Run `$ ./main --help` to open the help menu.

### Options:
`./main` also supports some options.
* `--total_games`: int - Determine the total **automatic** games that will be played (default: 1).
* `--render`: bool - Render game in the terminal when running the **automatic** game (default: False).


## Changelog:
    - Added gui for manual


    - For older changelogs see CHANGELOG.md


## To do:
- Add a gui for automatic
- Integrate keypress with tk gui

- Improve algorithm that solves the game
    - Add that the highest values must be in the same row on the bottom.
    - Add movement preferrences

    - Current winning chance of the algorithm is ~1/1000. A timer will be added later.
- Add highscore
- Add multithreading
- Rewrite code in rust? ^^
