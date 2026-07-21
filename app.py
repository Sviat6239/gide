import sys
from PySide6.QtCore import QSize
from PySide6.QtWidgets import QApplication, QMainWindow, QTextEdit, QVBoxLayout, QWidget

class MainWindow(QMainWindow):
    def __init__(self):
        super().__init__()
        self.setWindowTitle("G-IDE")
        self.setFixedSize(QSize(400, 300))

        self.setStyleSheet(
            "color: white;"
        )

        layout = QVBoxLayout()
        self.codearea = QTextEdit()
        layout.addWidget(self.codearea)
        
        container = QWidget()
        container.setLayout(layout)

        self.setCentralWidget(container)

app = QApplication(sys.argv)
window = MainWindow()
window.show()
app.exec()