from time import sleep
from typing import Final
from PySide6.QtWidgets import (
    QVBoxLayout,
    QWidget,
)
from PySide6.QtCore import QThreadPool, Qt, Slot

from src.game_displayer.board.board_widget import BoardWidget
from src.game_displayer.game.game_controls_widget import GameControlsWidget
from src.game_displayer.game.game_info_bar_widget import GameInfoBarWidget


class GameWidget(QWidget):

    BOARD_SIZE: Final[int] = 4

    def __init__(self, game: dict) -> None:
        super().__init__()
        self.__game = game
        self.__current_board_index = 0
        self.__game_controls = self.__setup_game_controls()
        self.__game_info_bar = GameInfoBarWidget()
        self.__board_widget = BoardWidget(self.BOARD_SIZE)
        self.__setup_game_layout()

        self.__thread_pool = QThreadPool()
        self.__thread_pool.setMaxThreadCount(1)

    def __display_full_game(self) -> None:
        """NOTE that this should be done on a different thread"""
        for board_index in range(self.__current_board_index, len(self.__game)):
            self.__current_board_index = board_index
            self.__update_displayed_board()
            sleep(0.1)
        self.__game_controls.setEnabled(True)
        self.__game_controls.set_controls_for_game_end()

    def __setup_game_layout(self):
        layout = QVBoxLayout(self)
        layout.addWidget(self.__game_info_bar)
        layout.addWidget(self.__board_widget)
        layout.addWidget(self.__game_controls)
        layout.setAlignment(Qt.AlignCenter)
        self.__update_displayed_board()

    def __setup_game_controls(self) -> GameControlsWidget:
        game_controls = GameControlsWidget()
        game_controls.startButtonClicked.connect(self.__on_start_clicked)
        game_controls.previousMoveButtonClicked.connect(
            self.__on_previous_button_clicked
        )
        game_controls.nextMoveButtonClicked.connect(self.__on_next_button_clicked)
        game_controls.goToStartButtonClicked.connect(
            self.__on_go_to_start_button_clicked
        )
        game_controls.goToEndButtonClicked.connect(self.__on_go_to_end_button_clicked)
        game_controls.set_controls_for_new_game()
        return game_controls

    def __update_displayed_board(self) -> None:
        current_move = self.__game[self.__current_board_index]
        self.__board_widget.boardUpdated.emit(current_move["board"])
        self.__game_info_bar.scoreUpdate.emit(current_move["score"])
        self.__game_info_bar.latestMovementUpdate.emit(current_move["performed move"])

    @Slot()
    def __on_start_clicked(self) -> None:
        self.__game_controls.setDisabled(
            True
        )  # Disabling controls is needed for threading!
        self.__thread_pool.start(self.__display_full_game)

    @Slot()
    def __on_previous_button_clicked(self) -> None:
        self.__current_board_index -= 1
        self.__update_displayed_board()

        if self.__current_board_index == 0:
            self.__game_controls.set_controls_for_new_game()
            return
        self.__game_controls.enable_all_controls()

    @Slot()
    def __on_next_button_clicked(self) -> None:
        self.__current_board_index += 1
        self.__update_displayed_board()

        if self.__current_board_index == len(self.__game) - 1:
            self.__game_controls.set_controls_for_game_end()
            return
        self.__game_controls.enable_all_controls()

    @Slot()
    def __on_go_to_start_button_clicked(self) -> None:
        self.__current_board_index = 0
        self.__update_displayed_board()
        self.__game_controls.set_controls_for_new_game()

    @Slot()
    def __on_go_to_end_button_clicked(self) -> None:
        self.__current_board_index = len(self.__game) - 1
        self.__update_displayed_board()
        self.__game_controls.set_controls_for_game_end()
