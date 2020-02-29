""" File contains a algorithm class that can solve the 2048 game """

class TfeAlgorithm:
    """ Twenty Forty Eight algorithm that can solve the 2048 game """

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
