import tiktoken
from typing import Protocol


class PricingCalculator(Protocol):
    def calculate_tokens(self, text: str) -> int:
        """Calculates the number of tokens in the given text."""
        pass

    def calculate_price(self, text: str) -> float:
        """Calculates the price for processing the given text."""
        pass



class OpenAIPricingCalculator:
    def __init__(self, model: str = "gpt-3.5-turbo", rate_per_1000_tokens: float = 0.002):
        self.tokenizer = tiktoken.encoding_for_model(model)
        self.rate_per_1000_tokens = rate_per_1000_tokens

    def calculate_tokens(self, text: str) -> int:
        """Calculates the number of tokens in the given text."""
        tokens = self.tokenizer.encode(text)
        return len(tokens)

    def calculate_price(self, text: str) -> float:
        """Calculates the price for processing the given text based on token count."""
        num_tokens = self.calculate_tokens(text)
        price = (num_tokens / 1000) * self.rate_per_1000_tokens
        return price