package unmarshal

import (
	"encoding/json"
	"invoice_create/src/models"
)

// Returns an Invoice struct
func UnmarshalInvoice(jsonData []byte) (models.Invoice, error) {
	var invoice models.Invoice

	err := json.Unmarshal(jsonData, &invoice)
	if err != nil {
		return models.Invoice{}, err
	}

	return invoice, nil
}
