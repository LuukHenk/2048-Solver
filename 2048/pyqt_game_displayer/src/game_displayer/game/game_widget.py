
from time import sleep  
from typing import Final, List
from PySide6.QtWidgets import (
    QVBoxLayout,
    QWidget,
)
from PySide6.QtCore import QThreadPool, Qt, Slot, Signal

from game_displayer.board.board_widget import BoardWidget, BoardType
from game_displayer.game.game_controls_widget import GameControlsWidget
from game_displayer.worker import Worker

class GameWidget(QWidget):
    
    BOARD_SIZE: Final[int] = 4
    
    
    def __init__(self, boards: BoardType) -> None:
        super().__init__()
        self.__boards: List[BoardType] = boards
        self.__thread_pool = QThreadPool()
        self.__game_controls = self.__setup_game_controls()
        self.__board_widget = BoardWidget(self.BOARD_SIZE)
        self.__setup_game_layout()
    
    def __display_game(self):
        for board in self.__boards:
            self.__board_widget.boardUpdate.emit(board)
            sleep(0.5)
        
    def __setup_game_layout(self):
        layout = QVBoxLayout(self)
        layout.addWidget(self.__board_widget)
        layout.addWidget(self.__game_controls)
        layout.setAlignment(Qt.AlignCenter)
    
    def __setup_game_controls(self) -> GameControlsWidget:
        game_controls = GameControlsWidget()
        game_controls.startClicked.connect(self.__on_start_clicked)
        return game_controls
        
    @Slot()
    def __on_start_clicked(self) -> None:
        worker = Worker(self.__display_game)
        self.__thread_pool.start(worker)
    