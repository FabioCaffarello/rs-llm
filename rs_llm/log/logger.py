import logging
import os

from pythonjsonlogger import jsonlogger


def setup_logging(
    module_name: str,
    propagate: bool = False,
    log_level: str = os.getenv("LOG_LEVEL", "INFO").upper()
) -> logging.Logger:
    log_handler = logging.StreamHandler()
    formatter = jsonlogger.JsonFormatter("%(levelname)s %(filename)s %(message)s", json_ensure_ascii=False)
    log_handler.setFormatter(formatter)

    logger = logging.getLogger(module_name)
    logger.addHandler(log_handler)
    logger.propagate = propagate
    logger.setLevel(logging.getLevelName(log_level))
    return logger
