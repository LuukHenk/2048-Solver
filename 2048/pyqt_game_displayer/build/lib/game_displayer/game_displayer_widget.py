
from pathlib import Path
from PySide6.QtWidgets import QMainWindow
from game_displayer.game.game_widget import GameWidget
from game_displayer.data_handler.load_game_from_json import load_json_to_dict

class GameDisplayerWidget(QMainWindow):
    """The main window of the 2048 game"""

    def __init__(self):
        super().__init__()
        games = load_json_to_dict("/home/zoutekaas/Dropbox/Active/2048-Solver/2048/data/results.json")
        game_boards = []
        for move in games[0]:
            game_boards.append(move["board"])

        self.setCentralWidget(GameWidget(game_boards))
        