from create_invoice.models import Invoice


def unmarshal_invoice(json_data: bytes) -> Invoice:
    return Invoice.model_validate_json(json_data)
