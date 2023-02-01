""" The main window of our game"""

from sys import (
    exit as sys_exit,
    argv,
)
from PySide6.QtWidgets import (
    QApplication, 
    QMainWindow,
)
from PySide6.QtCore import QThreadPool, QRunnable
from game_displayer.game_displayer_widget import GameDisplayerWidget

class GameRunnable(QRunnable):
    def run():
        app = QApplication(argv)
        window = GameDisplayerWidget()
        window.show()
        sys_exit(app.exec())
        
def run_main_window():
    """Runs the main window, kills everything that is in the main window when closed"""
    game_runnable = GameRunnable()
    QThreadPool.globalInstance().start(game_runnable)
    
