""" Rendering of the board """

def render_automatic_in_terminal(board, score, direction, total_moves, index):
    """ Rendering of the automatic game in the terminal """
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
