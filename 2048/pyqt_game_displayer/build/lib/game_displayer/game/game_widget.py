
from time import sleep  
from typing import Final, List
from PySide6.QtWidgets import (
    QVBoxLayout,
    QWidget,
)
from PySide6.QtCore import QThreadPool, Qt, Slot, Signal

from game_displayer.board.board_widget import BoardWidget, BoardType
from game_displayer.game.game_controls_widget import GameControlsWidget


class GameWidget(QWidget):
    
    BOARD_SIZE: Final[int] = 4
    
    
    def __init__(self, boards: BoardType) -> None:
        super().__init__()
        self.__boards = boards
        self.__current_board_index = 0
        self.__game_controls = self.__setup_game_controls()
        self.__board_widget = BoardWidget(self.BOARD_SIZE)
        self.__setup_game_layout()
        
        self.__thread_pool = QThreadPool()
        self.__thread_pool.setMaxThreadCount(1)

    def __display_full_game(self) -> None:
        """NOTE that this should be done on a different thread"""
        for board_index in range(self.__current_board_index, len(self.__boards)):
            self.__current_board_index = board_index
            self.__update_displayed_board()
            sleep(0.2)
        self.__game_controls.setEnabled(True) 
        self.__game_controls.on_game_end()
            
    def __setup_game_layout(self):
        layout = QVBoxLayout(self)
        layout.addWidget(self.__board_widget)
        layout.addWidget(self.__game_controls)
        layout.setAlignment(Qt.AlignCenter)
        self.__update_displayed_board()
    
    def __setup_game_controls(self) -> GameControlsWidget:
        game_controls = GameControlsWidget()
        game_controls.startButtonClicked.connect(self.__on_start_clicked)
        game_controls.previousMoveButtonClicked.connect(self.__on_previous_button_clicked)
        game_controls.nextMoveButtonClicked.connect(self.__on_next_button_clicked)
        game_controls.goToStartButtonClicked.connect(self.__on_go_to_start_button_clicked)
        game_controls.goToEndButtonClicked.connect(self.__on_go_to_end_button_clicked)
        game_controls.on_game_start()
        return game_controls
    
    def __update_displayed_board(self) -> None:
        current_board = self.__boards[self.__current_board_index] 
        self.__board_widget.boardUpdated.emit(current_board)
        
    @Slot()
    def __on_start_clicked(self) -> None:
        self.__game_controls.setDisabled(True) # Disabling controls is needed for threading!
        self.__thread_pool.start(self.__display_full_game)
    
    @Slot()
    def __on_previous_button_clicked(self) -> None:
        self.__current_board_index -= 1
        self.__update_displayed_board()
        
        if self.__current_board_index == 0:
            self.__game_controls.on_game_start()
            return
        self.__game_controls.enable_all_controls()
            

    @Slot()
    def __on_next_button_clicked(self) -> None:
        self.__current_board_index += 1
        self.__update_displayed_board()
        
        if self.__current_board_index == len(self.__boards) - 1:
            self.__game_controls.on_game_end()
            return
        self.__game_controls.enable_all_controls()
    
    @Slot()
    def __on_go_to_start_button_clicked(self) -> None:
        self.__current_board_index = 0
        self.__update_displayed_board()
        self.__game_controls.on_game_start()

    @Slot()
    def __on_go_to_end_button_clicked(self) -> None:
        self.__current_board_index = len(self.__boards) -1
        self.__update_displayed_board()
        self.__game_controls.on_game_end()