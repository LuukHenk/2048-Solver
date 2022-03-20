"""Widget for displaying game play"""

from PySide6 import QtCore, QtWidgets


from lib.presentation_layer.board import Board
from lib.business_layer.game_displayer import GameDisplayer
from lib.business_layer.python_game_engine.game_data_formats import MoveResult


class GameWidget(QtWidgets.QWidget):  # pylint: disable=R0903
    """The screen of the 2048 game"""

    update_board = QtCore.Signal(MoveResult)

    def __init__(self, thread_manager):
        """
        game (Game): The 2048 game runner
        """
        super().__init__()
        self.thread_manager = thread_manager
        self.start_button = None
        self.game_board = None
        self.__init_ui()

    def __init_ui(self):
        self.game_screen = QtWidgets.QVBoxLayout(self)
        # self.__add_game_information()
        self.__add_game_board()
        self.__add_start_button()

    def __add_game_board(self) -> None:
        self.game_board = Board()
        self.update_board.connect(self.game_board.update_cells)
        self.game_screen.addWidget(
            self.game_board,
            alignment=QtCore.Qt.AlignCenter,
        )

    def __add_start_button(self) -> None:
        self.start_button = QtWidgets.QPushButton("Start")
        self.start_button.clicked.connect(self.__on_start_safe)
        self.game_screen.addWidget(self.start_button)

    @QtCore.Slot()
    def __on_start_safe(self):
        self.start_button.setDisabled(True)
        self.thread_manager.start(self.__on_start)

    @QtCore.Slot()
    def __on_start(self):
        displayer = GameDisplayer()
        displayer.updated_move.connect(self.__on_new_move_result)
        displayer.display_games()

    @QtCore.Slot(MoveResult)
    def __on_new_move_result(self, move_result: MoveResult):
        self.update_board.emit(move_result)
