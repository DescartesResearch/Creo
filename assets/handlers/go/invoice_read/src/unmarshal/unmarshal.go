package unmarshal

import (
	"encoding/json"
	"invoice_read/src/models"
)

// Returns an Invoice struct
func UnmarshalInvoice(data map[string]any) (models.Invoice, error) {
	var invoice models.Invoice

	jsonData, err := json.Marshal(data)
	if err != nil {
		return models.Invoice{}, err
	}

	err = json.Unmarshal(jsonData, &invoice)
	if err != nil {
		return models.Invoice{}, err
	}

	return invoice, nil
}
