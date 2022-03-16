""" The main window of our game"""


from PySide6 import QtWidgets

from .game import GameWidget

class MainWindow(QtWidgets.QMainWindow):
    """The main window of the 2048 game"""
    def __init__(self):
        super().__init__()
        self.setCentralWidget(GameWidget())
        self.setStyleSheet("QLabel { background-color : #3C341F;}")
