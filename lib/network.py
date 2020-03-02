""" File contains a algorithms that can solve the 2048 game """
import random
from .game import Game

def determine_best_movement(board, possible_movements):
    """
    Determine the best move with the given board and the possible directions
    using the score, the position of the highest value

    """
    print("\nmove performed")
    game = Game()
    weights = {"left": [0], "right": [0], "down": [0], "up": [0]}
    combined_weights = {"left": 0, "right": 0, "down": 0, "up": 0}

    if len(possible_movements) > 1 and "up" in possible_movements:
        possible_movements.remove("up")
    # Determine best score
    scores_per_movement = determine_scores(board, possible_movements)

    # Add scores to weights
    for move in scores_per_movement:
        if scores_per_movement[move] > 0:
            weights[move][0] = round(scores_per_movement[move]/4)

    # TODO Keep the highest value in the corner

    # Find the highest value on the board

    highest_value_coordinates = get_highest_value_coordinates(board)
    highest_value = board[highest_value_coordinates[1]][highest_value_coordinates[0]]

    highest_in_corner = False
    if highest_value_coordinates[0] in (0, 3) and highest_value_coordinates[1] in (0, 3):
        highest_in_corner = True

    for move in possible_movements:
        new_highest_in_corner = False

        if move in ("left", "right") and highest_value_coordinates[1] in (0, 3):
            highest_row = board[highest_value_coordinates[1]]

            if move == "left":
                new_row = game.comparing(highest_row, False)
            elif move == "right":
                new_row = game.comparing(list(reversed(highest_row)), False)

            new_highest_in_corner = new_row[0] >= highest_value

        elif move in ("up", "down") and highest_value_coordinates[0] in (0, 3):
            rotated_board = rotate(board)
            highest_row = rotated_board[highest_value_coordinates[0]]

            if move == "up":
                new_row = game.comparing(highest_row, False)
            elif move == "down":
                new_row = game.comparing(list(reversed(highest_row)), False)

            new_highest_in_corner = new_row[0] >= highest_value

        print("{}\n{}\n{}\nbefore: {} - after: {}".format(
            move, board, highest_value_coordinates, highest_in_corner, new_highest_in_corner
            ))

            # TODO
            # give positive weight if new_highest_in_corner and NOT highest_in_corner
            # give no weight if new_highest_in_corner and highest_in_corner
            # give negative weight if NOT new_highest_in_corner and highest_in_corner
            # give no weight if both are False


        # if move == "up":
        #     board = rotate(board)
        #     print(board)


    # Determine combined weights
    for move in weights:
        combined_weights[move] = sum(weights[move])

    # Determine the hightest weights
    highest_weight_move = max(combined_weights, key=combined_weights.get)
    highest_weight = combined_weights[highest_weight_move]

    # Make a random choice if there is no weight
    if highest_weight == 0:
        best_movement = random.choice(possible_movements)
    # Else choose a move with the highest weight
    else:
        best_movements = []
        for move in combined_weights:
            if combined_weights[move] == highest_weight:
                best_movements.append(move)
        best_movement = random.choice(best_movements)

    return best_movement

def get_highest_value_coordinates(board):
    """
    Returns the location [x, y] of the highest value on the board
    Returns [0, 0] if all the values are 0
    """

    highest_value = -1
    for i, row in enumerate(board):
        for j, value in enumerate(row):
            if value > highest_value:
                highest_value = value
                highest_value_coordinates = [j, i]

    return highest_value_coordinates



def determine_scores(board, movements):
    """ Determine the score for each movement """

    scores = {"left": 0, "right": 0, "down": 0, "up": 0}

    for move in movements:
        if move == "left":
            for row in board:
                scores[move] += determine_score(row)

        elif move == "right":
            for row in board:
                scores[move] += determine_score(list(reversed(row)))

        elif move == "down":
            board = rotate(board)
            for row in board:
                scores[move] += determine_score(list(reversed(row)))

        elif move == "up":
            board = rotate(board)
            for row in board:
                scores[move] += determine_score(row)
        else:
            print("E: Can not detect score!")

    return scores

def rotate(board):
    """ Rotates the board by switching the x and y positions"""

    board_range = range(len(board))
    out = [[0 for _ in board_range] for _ in board_range]

    for y_pos, row in enumerate(board):
        for x_pos, value in enumerate(row):
            out[x_pos][y_pos] = value

    return out

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
