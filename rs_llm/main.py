import json
from log.logger import setup_logging
from pathlib import Path
from loader.pdf_loader import PDFTextExtractor
from pricing.token import OpenAIPricingCalculator

logger = setup_logging(__name__)


def _get_config():
    with open("/app/configs/config.json", "r") as f:
        return json.load(f)

def get_file_name():
    config = _get_config()
    return config["file_name"]

if __name__ == "__main__":
    file_name = get_file_name()
    logger.info(f"File name: {file_name}")
    file_path = Path(f"/app/data/{file_name}")
    extractor = PDFTextExtractor(file_path)
    text = extractor.extract_text()
    logger.info(f"Text extracted from the PDF file: {text[100:]}")

    pricing_calculator = OpenAIPricingCalculator()
    price = pricing_calculator.calculate_price(text)
    logger.info(f"Price for processing the text: ${price}")


