""" File contains a algorithms that can solve the 2048 game """
import random
from .game import Game
game = Game()

def determine_best_movement(board, possible_movements, print_weights=False):
    """
    Determine the best move with the given board and the possible directions
    using the score, the position of the highest value

    """

    if "up" in possible_movements and len(possible_movements) > 1:
        possible_movements.remove("up")
    # Generate weight per possible movement which will determine the final score
    weights = {move:0 for move in possible_movements}

    # Add scores to weights
    weights = determine_score_weights(board, weights)
    weights = determine_value_position_weights(board, weights)

    if print_weights:
        score_weights = determine_score_weights(board, {move:0 for move in possible_movements})
        empty_weights = {move:0 for move in possible_movements}
        value_position_weights = determine_value_position_weights(board, empty_weights)
        print(
            f"""
            \rScore weights: {score_weights}
            \rValue position weights: {value_position_weights}
            \rTotal weights: {weights}
            """
        )

    # Determine the highest weight (int)
    highest_weight = weights[max(weights, key=weights.get)]

    # Return a random move from weights with the highest weight
    return random.choice([move for move in weights if weights[move] == highest_weight])

def determine_value_position_weights(board, weights, factor=1):
    """
    Determine the position of the highest value on the board,
    determines if this value is on a strategic position (corner, side),
    and translates this to a weight
    """

    #TODO add weights when on the side
    # Determine the coordinates of the highest value on the board
    highest_value = board[0][0]
    highest_value_coordinates = [0, 0]
    for i, row in enumerate(board):
        for j, value in enumerate(row):
            if value >= highest_value:
                highest_value = value
                highest_value_coordinates = [j, i]

    if highest_value != 0:
        # Determine if the highest value on the board is in a corner
        corners = (0, len(board)-1)
        highest_in_corner = (highest_value_coordinates[0] in corners
                             and highest_value_coordinates[1] in corners)

        # Determine if the highest value is in top/bottom of the column
        highest_on_side_column = True if highest_value_coordinates[1] in corners else False
        # Determine if the highest value is in top/bottom of the row
        highest_on_side_row = True if highest_value_coordinates[0] in corners else False

        for move in weights:
            # Determine if the row (for left and right movement)
            # or column (for down and up movement)
            # with the highest value is on the side before and after the movement
            if move in ("left", "right") and highest_on_side_column:
                highest_row = board[highest_value_coordinates[1]]
            elif move in ("up", "down") and highest_on_side_row:
                rotated_board = rotate(board)
                highest_row = rotated_board[highest_value_coordinates[0]]
            else:
                continue

            # If the highest value is on the side, perform a movement with this row/column
            if move in ("left", "up"):
                new_row = game.comparing(highest_row, False)
            elif move in ("right", "down"):
                new_row = game.comparing(list(reversed(highest_row)), False)

            # Check if the highest value is in a corner after the performed movement
            # give positive weight if the highest value was not in a corner but is now
            if new_row[corners[0]] >= highest_value or new_row[corners[1]] >= highest_value:
                if not highest_in_corner:
                    weights[move] += round(highest_value/2) * factor
            # give a negative weight if the highest value was in a corner but is not anymore
            else:
                if highest_in_corner:
                    weights[move] -= round(highest_value/2) * factor

    return weights

def determine_score_weights(board, weights, factor=1):
    """ Determine the score weights for each possible movement """

    for move in weights:
        if move in ("down", "up"):
            board = rotate(board)

        score_weight = 0
        if move in ("left", "up"):
            score_weight = sum([determine_score(row) for row in board])
        elif move in ("right", "down"):
            score_weight = sum([determine_score(list(reversed(row))) for row in board])

        if score_weight != 0:
            weights[move] += round(score_weight/4) * factor

    return weights

def determine_score(sample):
    """ Determine the score for a left movement on the board"""

    score = 0
    inp = list(filter(lambda value: value != 0, sample))

    while len(inp) > 1:
        first_value = inp.pop(0)
        second_value = inp.pop(0)

        if first_value == second_value:
            score += first_value*2
        else:
            inp.insert(0, second_value)

    return score

def rotate(board):
    """ Rotates the board by switching the x and y positions"""

    board_range = range(len(board))
    out = [[0 for _ in board_range] for _ in board_range]

    for y_pos, row in enumerate(board):
        for x_pos, value in enumerate(row):
            out[x_pos][y_pos] = value

    return out

