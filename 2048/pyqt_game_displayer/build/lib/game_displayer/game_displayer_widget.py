
from PySide6.QtWidgets import QMainWindow
from game_displayer.game.game_widget import GameWidget

class GameDisplayerWidget(QMainWindow):
    """The main window of the 2048 game"""

    def __init__(self):
        super().__init__()
        board_1 = [
            ["0", "0", "0", "0"],
            ["0", "0", "2", "0"],
            ["0", "0", "2", "0"],
            ["0", "0", "0", "0"]
        ]
        board_2 = [
            ["0", "0", "0", "0"],
            ["0", "2", "0", "0"],
            ["0", "0", "0", "0"],
            ["0", "0", "4", "0"]
        ]
        board_3 = [
            ["0", "0", "0", "0"],
            ["0", "0", "0", "0"],
            ["0", "0", "0", "2"],
            ["0", "0", "4", "4"]
        ]
        board_4 = [
            ["0", "0", "0", "0"],
            ["0", "0", "0", "2"],
            ["0", "0", "0", "2"],
            ["0", "0", "0", "8"]
        ]
        board_5 = [
            ["0", "0", "0", "0"],
            ["0", "0", "0", "2"],
            ["0", "0", "0", "4"],
            ["0", "0", "0", "8"]
        ]
                                        
        game_boards = [
            board_1,
            board_2,
            board_3,
            board_4,
            board_5,
            board_1,
            board_2,
            board_3,
            board_4,
            board_5,
            board_1,
            board_2,
            board_3,
            board_4,
            board_5,
            board_1,
            board_2,
            board_3,
            board_4,
            board_5,
            board_1,
            board_2,
            board_3,
            board_4,
            board_5,
            board_1,
            board_2,
            board_3,
            board_4,
            board_5,
            board_1,
            board_2,
            board_3,
            board_4,
            board_5,
        ]
        self.setCentralWidget(GameWidget(game_boards))
        