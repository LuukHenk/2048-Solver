import random
import time
from lib.board import Board

class Game:
    def __init__(self):
        self.board = Board()
        self.score = 0
        self.insertNumber()
        self.insertNumber()

    def setMovement(self, move):
        if move == 'left' : self.leftMove()
        if move == 'right': self.rightMove()
        if move == 'down' : self.downMove()
        if move == 'up'   : self.upMove()
        
    def insertNumber(self):
        if (self.board.full(self.board.board) == False):
            position = random.choice(self.board.emptyTiles())
            value = 2
            if(random.randint(1, 10) == 10):
                value = 4
            return self.board.set(value, position['y'], position['x'])

    def gameOver(self, possibleMovements):
        return (True if len(possibleMovements) == 0 else False)

    def leftMove(self):
        self.board.replace([self.comparing(row) for row in self.board.board])
        self.insertNumber()

    def rightMove(self):
        m = [list(reversed(self.comparing(list(reversed(row))))) for row in self.board.board]
        self.board.replace(m)
        self.insertNumber()

    def downMove(self):
        self.board.rotate()
        self.rightMove()
        self.board.rotate()

    def upMove(self):
        self.board.rotate()
        self.leftMove()
        self.board.rotate()

    def comparing(self, sample):
        inp = list(filter(lambda value: value != 0, sample))
        out = []

        while (len(inp) > 0):
            a = inp.pop(0)
            m = (inp.pop(0) if (len(inp) >= 1) else None)

            if (a == m):
                inp.insert(0, a * 2)
                self.score += a * 2
            else:
                out.append(a)
                if (m == None):break
                inp.insert(0, m)

        zerosToAdd = len(sample) - len(out)
        while (zerosToAdd > 0):
            zerosToAdd -= 1
            out.append(0)
        return out
