
import sys
from PySide6 import QtCore, QtWidgets

from .board import Board

# setting path
sys.path.append('../business_layer')
from business_layer.game import Game

class GameWidget(QtWidgets.QWidget):
    """The screen of the 2048 game"""
    def __init__(self):
        """
        game (Game): The 2048 game runner
        """
        super().__init__()
        self.game_screen = QtWidgets.QVBoxLayout(self)
        # self.game_engine = game_engine()
        self.game_screen.addWidget(
            Board([[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0],[0, 0, 0, 0]]),
            # Board(self.game_engine.board),
            alignment = QtCore.Qt.AlignCenter
        )
        self.setStyleSheet("QLabel { background-color : #3C341F;}")
        self.game_screen.addWidget(QtWidgets.QPushButton("Start"))
