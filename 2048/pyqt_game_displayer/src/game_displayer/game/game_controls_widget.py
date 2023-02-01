
from typing import Final
from PySide6.QtWidgets import (
    QHBoxLayout,
    QWidget,
    QPushButton,
)
from PySide6.QtCore import Qt, Signal, Slot


class GameControlsWidget(QWidget):
    
    startButtonClicked = Signal()
    previousMoveButtonClicked = Signal()
    nextMoveButtonClicked = Signal()
    goToStartButtonClicked = Signal()
    goToEndButtonClicked = Signal()
    
    START_TEXT: Final[str] = "Start"
    PREVIOUS_MOVE_TEXT: Final[str] = "<"
    NEXT_MOVE_TEXT: Final[str] = ">"
    GO_TO_START_TEXT: Final[str] = "<<<"
    GO_TO_END_TEXT: Final[str] = ">>>"
    
    def __init__(self) -> None:
        super().__init__()
        self.__start_button = self.__create_start_button()
        self.__previous_move_button = self.__create_previous_move_button()
        self.__next_move_button = self.__create_next_move_button()
        self.__go_to_start_button = self.__create_go_to_start_button()
        self.__go_to_end_button = self.__create_go_to_end_button()
        self.__create_game_controls()
    
    def enable_all_controls(self) -> None:
        self.__start_button.setEnabled(True)
        self.__previous_move_button.setEnabled(True)
        self.__next_move_button.setEnabled(True)
        self.__go_to_start_button.setEnabled(True)
        self.__go_to_end_button.setEnabled(True)
        
    def set_controls_for_new_game(self) -> None:
        self.__start_button.setEnabled(True)
        self.__previous_move_button.setDisabled(True)
        self.__next_move_button.setEnabled(True)
        self.__go_to_start_button.setDisabled(True)
        self.__go_to_end_button.setEnabled(True)        
    
    def set_controls_for_game_end(self) -> None:
        self.__start_button.setDisabled(True)
        self.__previous_move_button.setEnabled(True)
        self.__next_move_button.setDisabled(True)
        self.__go_to_start_button.setEnabled(True)
        self.__go_to_end_button.setDisabled(True)    
        
    def __create_game_controls(self) -> None:
        layout = QHBoxLayout(self)
        layout.addWidget(self.__go_to_start_button)
        layout.addWidget(self.__previous_move_button)
        layout.addWidget(self.__start_button)
        layout.addWidget(self.__next_move_button)
        layout.addWidget(self.__go_to_end_button)
        layout.setAlignment(Qt.AlignCenter)
        
    def __create_start_button(self) -> QPushButton:
        start_button = QPushButton(self.START_TEXT)
        start_button.clicked.connect(self.startButtonClicked)
        return start_button
    
    def __create_previous_move_button(self) -> QPushButton:
        previous_move_button = QPushButton(self.PREVIOUS_MOVE_TEXT)
        previous_move_button.clicked.connect(self.previousMoveButtonClicked)
        return previous_move_button
    
    def __create_next_move_button(self) -> QPushButton:
        next_move_button = QPushButton(self.NEXT_MOVE_TEXT)
        next_move_button.clicked.connect(self.nextMoveButtonClicked)
        return next_move_button
    
    def __create_go_to_start_button(self) -> QPushButton:
        go_to_start_button = QPushButton(self.GO_TO_START_TEXT)
        go_to_start_button.clicked.connect(self.goToStartButtonClicked)
        return go_to_start_button
    
    def __create_go_to_end_button(self) -> QPushButton:
        go_to_end_button = QPushButton(self.GO_TO_END_TEXT)
        go_to_end_button.clicked.connect(self.goToEndButtonClicked)
        return go_to_end_button