import os
import argparse
import sys
import termios
import fcntl

def argument_handler():
    """ handles input arguments """

    parser = argparse.ArgumentParser()
    parser.add_argument(
        "game_mode",
        type=str,
        choices=["manual", "automatic"],
        help="Play the game in manual mode or automatic mode"
    )

    parser.add_argument(
        "--render",
        type=bool,
        choices=[True, False],
        default=False,
        help="Render game in the terminal when running the automatic game (default: False)"
    )

    parser.add_argument(
        "--total_games",
        type=int,
        default=1,
        help="int - The total games that will be played (default: 1)"
    )
    return parser.parse_args()

def read_single_keypress():
    """Waits for a single keypress on stdin.

    This is a silly function to call if you need to do it a lot because it has
    to store stdin's current setup, setup stdin for reading single keystrokes
    then read the single keystroke then revert stdin back after reading the
    keystroke.

    Returns the character of the key that was pressed (zero on
    KeyboardInterrupt which can happen when a signal gets handled)

    Function obtained from:
    https://stackoverflow.com/questions/983354/how-do-i-make-python-wait-for-a-pressed-key
    """

    f_d = sys.stdin.fileno()
    # save old state
    flags_save = fcntl.fcntl(f_d, fcntl.F_GETFL)
    attrs_save = termios.tcgetattr(f_d)
    # make raw - the way to do this comes from the termios(3) man page.
    attrs = list(attrs_save) # copy the stored version to update
    # iflag
    attrs[0] &= ~(termios.IGNBRK | termios.BRKINT | termios.PARMRK
                  | termios.ISTRIP | termios.INLCR | termios. IGNCR
                  | termios.ICRNL | termios.IXON)
    # oflag
    attrs[1] &= ~termios.OPOST
    # cflag
    attrs[2] &= ~(termios.CSIZE | termios. PARENB)
    attrs[2] |= termios.CS8
    # lflag
    attrs[3] &= ~(termios.ECHONL | termios.ECHO | termios.ICANON
                  | termios.ISIG | termios.IEXTEN)
    termios.tcsetattr(f_d, termios.TCSANOW, attrs)
    # turn off non-blocking
    fcntl.fcntl(f_d, fcntl.F_SETFL, flags_save & ~os.O_NONBLOCK)
    # read a single keystroke
    try:
        ret = sys.stdin.read(1) # returns a single character
    except KeyboardInterrupt:
        ret = 0
    finally:
        # restore old state
        termios.tcsetattr(f_d, termios.TCSAFLUSH, attrs_save)
        fcntl.fcntl(f_d, fcntl.F_SETFL, flags_save)
    return ret

def render_automatic_in_terminal(board, score, direction, total_moves, index):
    total_board = build_fancy_board(board)

    print(
        f"""
\n\n\n\n\n\n\n\n\n
{total_board}
Current game: {index}
Movement: {direction}
Score: {score}
Moves: {total_moves}
    """
    )

def render_manual_in_terminal(keys, board, score, direction, total_moves):
    """ Rendering of the manual game in the terminal """

    total_board = build_fancy_board(board)

    # Print the board on the screen
    print(
        f"""
\n\n\n\n\n\n\n\n\n\n\n\n\n
____________________________________________

- Press {keys['left']} to move left
- Press {keys['right']} to move right
- Press {keys['down']} to move down
- Press {keys['up']} to move up
- Press {keys['quit']} to quit the game

____________________________________________
\n\n\n\n\n\n\n\n\n
{total_board}
Movement: {direction}
Score: {score}
Moves: {total_moves}
        """
    )

def build_fancy_board(board):
    """ Build a fancy terminal board of the board list"""
    # Set the width of the board
    board_spacing = 10

    # Build tile spacing
    tile_spacing = "".join([" " for _ in range(board_spacing)])
    tile_spacing = "||".join([tile_spacing, tile_spacing, tile_spacing, tile_spacing])

    # Build midline
    midline = "".join(["_" for _ in range(board_spacing)])
    midline = "00".join([midline, midline, midline, midline])

    # Build the board
    total_board = ""
    for i, row in enumerate(board):

        # Build the tiles
        tiles = list(map(lambda n: str(" " if n == 0 else n).center(board_spacing, " "), row))
        line = "||".join(tiles)

        if i == 0:
            total_board = "\n".join([tile_spacing, line, tile_spacing])
        else:
            total_board = "\n".join([total_board, midline, tile_spacing, line, tile_spacing])

    return total_board
