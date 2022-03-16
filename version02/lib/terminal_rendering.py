""" Rendering of the board """

def render_automatic_in_terminal(direction, game, index):
# def render_automatic_in_terminal(board, score, direction, total_moves, index):
#     """ Rendering of the automatic game in the terminal """
    total_board = build_fancy_board(game.board.board, game.board.size)
    print(
        f"""
        \r\n\n\n\n\n\n\n\n\n
        \r{total_board}
        \n
        \r------------------------------------------------\n
        \rCurrent game: {index}
        \rMovement: {direction}
        \rScore: {game.score}
        \rMoves: {game.moves}"""
    )

def render_manual_in_terminal(keys, direction, game):
    """ Rendering of the manual game in the terminal """
    total_board = build_fancy_board(game.board.board, game.board.size)
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
    \n\n\n\n\n\n\n\n\n{total_board}
    Movement: {direction}
    Score: {game.score}
    Moves: {game.moves}
            """
    )

def build_fancy_board(board, board_size):
    """ Build a fancy terminal board of the board list"""
    # Set the width of the board
    board_spacing = 10

    # Build tile spacing
    tile_spacing = "".join([" " for _ in range(board_spacing)])
    tile_spacing = "||".join([tile_spacing for _ in range(board_size)])

    # Build midline
    midline = "".join(["_" for _ in range(board_spacing)])
    midline = "00".join([midline for _ in range(board_size)])

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
