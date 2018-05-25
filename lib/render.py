class Render:
    def board(self, board, score):
        for row in board:
            print('|'.join(list(map(lambda n: str(' ' if n == 0 else n).center(2, ' '), row))))
        print(' '.join(['Score: ', str(score)]))
        print("\n____________\n")
        
    def move(self, move):
        print(' '.join(['Movement:',  move.rjust(2)]))
        
