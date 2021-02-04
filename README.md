# Twenty-Forty-Eight - The Remake
#### Version 0.2.0
2048 is a simple numberbased board game with a 4x4 board. All the values on the board are two or a squired form of two. Values can be merged when equal by moving the values over the board using [w, s, a, d]. The goal of the game is to obtain a value of 2048. More info about the game [here](https://en.wikipedia.org/wiki/2048_(video_game)).

This program lets you play the 2048 game in the terminal ~~and in a gui~~. Or let the computer brain work and make the build-in algorithm solve the game for you in automatic mode.

This is an updated version of my very first programming project with [Sidney Liebrand](https://github.com/SidOfc).

## Quick start
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
* `--render`: bool - The type of game rendering {None, terminal, gui} (default: terminal).
* `--board_size`: int - The grid size of the board (default: 4)


## Changelog:
    - Added gui to the automatic game mode
    - For older changelogs see CHANGELOG.md
