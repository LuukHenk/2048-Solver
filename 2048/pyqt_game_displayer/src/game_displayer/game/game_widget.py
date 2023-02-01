

from PySide6.QtWidgets import (
    QVBoxLayout,
    QWidget,
)
from PySide6.QtCore import QThreadPool, Qt

from game_displayer.board.board_widget import BoardWidget, BoardType
from game_displayer.game.game_controls_widget import GameControlsWidget

class GameWidget(QWidget):
    def __init__(self, boards: BoardType) -> None:
        super().__init__()
        self.__thread_manager = QThreadPool()
        self.__game_controls = GameControlsWidget()
        self.__setup_game_layout(boards[0])
    
    def __setup_game_layout(self, first_board: BoardType):
        layout = QVBoxLayout(self)
        layout.addWidget(BoardWidget(first_board))
        layout.addWidget(self.__game_controls)
        layout.setAlignment(Qt.AlignCenter)