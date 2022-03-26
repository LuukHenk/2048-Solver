"""Widget for displaying game play"""

from random import choice
from time import sleep

from PySide6.QtWidgets import (  # pylint: disable=E0611
    QWidget,
    QVBoxLayout,
    QPushButton,
    QLayout,
)
from PySide6.QtCore import Signal, Slot, Qt, QThreadPool  # pylint: disable=E0611

from lib.utils.json_file import JsonProcessor
from lib.data_layer.game_data_formats import (
    GameResult,
    MoveResult,
)

from lib.presentation_layer.board import Board


class GameWidget(QWidget):  # pylint: disable=R0903
    """The screen of the 2048 game"""

    update_board = Signal(MoveResult)
    game_finished = Signal()

    def __init__(self, thread_manager: QThreadPool, fps: int):
        """ """
        super().__init__()
        self.__thread_manager = thread_manager
        self.__sleep_time = 1 / fps
        self.__games_data = self.__load_games()

        self.__init_ui()

    def __init_ui(self):
        """Creates the UI"""
        self.game_screen = QVBoxLayout(self)
        # self.__add_game_information()
        self.__add_game_board()
        self.__add_start_button()
        self.game_finished.connect(self.__on_finished_game)

    def __add_game_board(self) -> None:
        game_board = Board()
        self.update_board.connect(game_board.update_cells)
        self.game_screen.addWidget(
            game_board,
            alignment=Qt.AlignCenter,
        )

    def __add_start_button(self) -> None:
        start_button = QPushButton("Start")
        start_button.setObjectName("start_button")
        start_button.clicked.connect(self.__on_start_safe)
        self.game_screen.addWidget(start_button)

    @Slot()
    def __on_finished_game(self):
        self.__get_widget_from_layout(self.game_screen, "start_button").setEnabled(True)

    @Slot()
    def __on_start_safe(self):
        self.__get_widget_from_layout(self.game_screen, "start_button").setDisabled(
            True
        )
        self.__thread_manager.start(self.__display_game)

    def __display_game(self):
        for move in choice(self.__games_data):
            self.update_board.emit(move)
            sleep(self.__sleep_time)
        self.game_finished.emit()

    @staticmethod
    def __load_games() -> GameResult:
        json_processor = JsonProcessor()
        return json_processor.load_json_to_dict()

    @staticmethod
    def __get_widget_from_layout(layout: QLayout, name: str) -> QWidget:
        for i in range(layout.count()):
            widget = layout.itemAt(i).widget()
            if widget.objectName() == name:
                return widget
        return None
