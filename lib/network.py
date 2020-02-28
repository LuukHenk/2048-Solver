""" File contains a algorithm class that can solve the 2048 game """

class TfeAlgorithm:
    """ Twenty Forty Eight algorithm that can solve the 2048 game """

    def possible_directions(self, board):
        """ Determines which moves are possible by performing the movement """

        return [move for move in ["left", "right", "down", "up"] if self.mergeable(board, move)]

    def mergeable(self, board, move):
        """ Tries to perform a movement and returns if that movement is possible """

        if move == 'left':
            return any([self.move_possible(row) for row in board])

        if move == 'right':
            return any([self.move_possible(list(reversed(row))) for row in board])

        board = self.rotate(board)
        if move == 'down':
            return any([self.move_possible(list(reversed(row))) for row in board])

        if move == 'up':
            return any([self.move_possible(row) for row in board])

        return None

    def move_possible(self, row):
        """ Determines if a left movement is possible """

        if row[0] == 0:
            return True

        row = list(filter(lambda value: value != 0, row))
        for i in range(1, len(row)):
            if row[i-1] == row[i]:
                return True

        return False

    def rotate(self, board):
        """ Rotates the board by switching the x and y positions"""

        board_range = range(len(board))
        out = [[0 for _ in board_range] for _ in board_range]

        for y_pos, row in enumerate(board):
            for x_pos, value in enumerate(row):
                out[x_pos][y_pos] = value

        return out

    # def determineMovementSucces(self, boardBefore, boardAfter, move):
    #     movementScore = [0, 0, 0]
    #     movementScore[0] = self.changeInEmptyTiles(boardBefore, boardAfter)
    #     print(boardBefore)
    #     print(boardAfter)
    #     print(movementScore)

    # def changeInEmptyTiles(self, boardBefore, boardAfter):
    #     tilesBefore = 0
    #     tilesAfter  = 0
    #     for x, y in zip(boardBefore, boardAfter):
    #         tilesBefore = tilesBefore + x.count(0)
    #         tilesAfter  = tilesAfter  + y.count(0)
    #     return 1 - tilesBefore / float(tilesAfter)
