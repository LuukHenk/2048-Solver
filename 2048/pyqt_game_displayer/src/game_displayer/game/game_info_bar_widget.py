
from typing import Final

from PySide6.QtWidgets import QWidget, QHBoxLayout, QLabel
from PySide6.QtCore import Qt, Signal, Slot

class GameInfoBarWidget(QWidget):
    
    SCORE_TEXT: Final[str] = "Score: {}"
    LATEST_MOVEMENT_TEXT: Final[str] = "Latest movement: {}"
    scoreUpdate = Signal(str)
    latestMovementUpdate = Signal(str)
    
    def __init__(self) -> None:
        super().__init__()
        self.__score_widget = self.__create_score_widget()
        self.__latest_movement_widget = self.__create_latest_movement_widget()
        self.__create_widget_layout()
        
        self.scoreUpdate.connect(self.__on_score_update)
        self.latestMovementUpdate.connect(self.__on_latest_movement_update)
        
        
    def __create_widget_layout(self):
        layout = QHBoxLayout(self)
        layout.addWidget(self.__score_widget)
        layout.addWidget(self.__latest_movement_widget)
        layout.setAlignment(Qt.AlignCenter)
        
    def __create_score_widget(self) -> QLabel:
        return QLabel(self.SCORE_TEXT.format("0"))
    
    def __create_latest_movement_widget(self) -> QLabel:
        return QLabel(self.LATEST_MOVEMENT_TEXT.format("None"))
    
    @Slot(str)
    def __on_score_update(self, new_score: str) -> None:
        self.__score_widget.setText(self.SCORE_TEXT.format(new_score))
        
    @Slot(str)
    def __on_latest_movement_update(self, latest_movement: str) -> None:
        self.__latest_movement_widget.setText(self.LATEST_MOVEMENT_TEXT.format(latest_movement))