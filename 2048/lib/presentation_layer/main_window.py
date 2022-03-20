""" The main window of our game"""

import sys
from PySide6 import QtWidgets
from PySide6.QtCore import QThreadPool  # pylint: disable=no-name-in-module

from lib.presentation_layer.game import GameWidget


class MainWindow(QtWidgets.QMainWindow):  # pylint: disable=R0903
    """The main window of the 2048 game"""

    def __init__(self, sleep_time):
        super().__init__()
        self.thread_manager = QThreadPool()
        self.setCentralWidget(GameWidget(self.thread_manager, sleep_time))
        self.setStyleSheet("QLabel { background-color : #3C341F;}")


def run_main_window(sleep_time):
    """Runs the main window, kills everything that is in the main window when closed"""
    app = QtWidgets.QApplication(sys.argv)
    window = MainWindow(sleep_time)
    window.show()

    sys.exit(app.exec())
