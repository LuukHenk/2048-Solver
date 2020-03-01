""" File contains a algorithms that can solve the 2048 game """
import random

def determine_best_movement(board, possible_movements):
    """
    Determine the best move with the given board and the possible directions
    using the score ................

    """
    scores_per_movement = determine_scores(board, possible_movements)
    highest_score_move = max(scores_per_movement, key=scores_per_movement.get)

    if scores_per_movement[highest_score_move] == 0:
        best_movement = random.choice(possible_movements)
    else:
        best_movement = highest_score_move

    return best_movement



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
