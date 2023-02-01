from time import sleep

from PySide6.QtCore import QRunnable, Slot
from game_displayer.game.game_widget_worker_signals import GameWidgetWorkerSignals

class GameWidgetWorker(QRunnable):
    '''
    Worker thread

    Inherits from QRunnable to handler worker thread setup, signals and wrap-up.

    :param callback: The function callback to run on this worker thread. Supplied args and
                     kwargs will be passed through to the runner.
    :type callback: function
    :param args: Arguments to pass to the callback function
    :param kwargs: Keywords to pass to the callback function

    '''

    def __init__(self, boards):
        super(GameWidgetWorker, self).__init__()
        self.__boards = boards
        self.signal = GameWidgetWorkerSignals()

    @Slot()
    def run(self):
        '''
        Initialise the runner function with passed args, kwargs.
        '''
        for board in self.__boards:
            self.signal.emit(board)
            sleep(0.5)