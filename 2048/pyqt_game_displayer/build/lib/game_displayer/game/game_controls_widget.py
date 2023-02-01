
from typing import Final
from PySide6.QtWidgets import (
    QHBoxLayout,
    QWidget,
    QPushButton,
)
from PySide6.QtCore import Qt, Signal, Slot


class GameControlsWidget(QWidget):
    
    startClicked = Signal()
    
    START_TEXT: Final[str] = "Start"
    PAUSE_TEXT: Final[str] = "Pause"
    
    def __init__(self) -> None:
        super().__init__()
        self.__start_button = self.__create_start_button()
        self.__create_game_controls()
        
    def __create_game_controls(self) -> None:
        layout = QHBoxLayout(self)
        layout.addWidget(self.__start_button)
        layout.setAlignment(Qt.AlignCenter)
        
    def __create_start_button(self) -> QPushButton:
        button = QPushButton(self.START_TEXT)
        button.clicked.connect(self.__on_start)
        return button
    
    @Slot()
    def __on_start(self):
        self.__start_button.setDisabled(True)
        self.startClicked.emit()