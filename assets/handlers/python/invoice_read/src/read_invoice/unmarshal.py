from typing import Any
from read_invoice.models import Invoice


def unmarshal_invoice(data: dict[str, Any]) -> Invoice:
    return Invoice.model_validate(data)
