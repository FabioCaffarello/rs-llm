import PyPDF2
from pathlib import Path
from typing import Protocol

class TextExtractor(Protocol):
    def extract_text(self) -> str:
        """Extracts text from the source and returns it as a string."""
        pass

class PDFTextExtractor:
    def __init__(self, pdf_path: Path):
        self.pdf_path = pdf_path

    def extract_text(self) -> str:
        """Extracts text from the PDF file specified by pdf_path."""
        # Open the PDF file in binary read mode
        with open(self.pdf_path, 'rb') as file:
            reader = PyPDF2.PdfReader(file)
            text = ""
            # Iterate through the pages and extract text
            for page_num in range(len(reader.pages)):
                page = reader.pages[page_num]
                extracted_text = page.extract_text()
                if extracted_text:
                    # Ensure the text is UTF-8 encoded
                    extracted_text = extracted_text.encode('utf-8', errors='replace').decode('utf-8')
                    text += extracted_text
        return text