
from PySide6.QtWidgets import QMainWindow
from game_displayer.game.game_widget import GameWidget

class GameDisplayerWidget(QMainWindow):
    """The main window of the 2048 game"""

    def __init__(self):
        super().__init__()
        board = [
            ["0", "0", "0", "0"],
            ["0", "0", "2048", "0"],
            ["0", "0", "2", "0"],
            ["0", "0", "0", "0"]
        ]
        game_boards = [board, board, board, board]
        self.setCentralWidget(GameWidget(game_boards))
        