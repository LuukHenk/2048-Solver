""" The main window of our game"""

import sys
from PySide6 import QtWidgets

from lib.presentation_layer.game import GameWidget


class MainWindow(QtWidgets.QMainWindow):  # pylint: disable=R0903
    """The main window of the 2048 game"""

    def __init__(self):
        super().__init__()
        self.setCentralWidget(GameWidget())
        self.setStyleSheet("QLabel { background-color : #3C341F;}")


def run_main_window():
    """Runs the main window, kills everything that is in the main window when closed"""
    app = QtWidgets.QApplication(sys.argv)
    window = MainWindow()
    window.show()

    sys.exit(app.exec())
