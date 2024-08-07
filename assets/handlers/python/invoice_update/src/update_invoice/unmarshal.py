from update_invoice.models import UpdateInvoice


def unmarshal_invoice(json_data: bytes) -> UpdateInvoice:
    return UpdateInvoice.model_validate_json(json_data)
