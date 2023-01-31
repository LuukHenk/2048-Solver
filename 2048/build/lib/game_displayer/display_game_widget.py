""" The main window of our game"""

from sys import exit as sys_exit, argv
from PySide6 import QtWidgets

from game_displayer.board.board_widget import BoardWidget


class MainWindow(QtWidgets.QMainWindow):
    """The main window of the 2048 game"""

    def __init__(self):
        super().__init__()
        test_board = [
            ["1", "2", "3", "4"],
            ["1", "2", "3", "4"],
            ["1", "2", "3", "4"],
            ["1", "2", "3", "4"],
        ]
        self.setCentralWidget(BoardWidget(test_board))


def run_main_window():
    """Runs the main window, kills everything that is in the main window when closed"""
    app = QtWidgets.QApplication(argv)
    window = MainWindow()
    window.show()

    sys_exit(app.exec())
