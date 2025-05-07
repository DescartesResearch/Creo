package unmarshal

import (
	"encoding/json"
	"invoice_update/src/models"
)

// Returns an Invoice struct
func UnmarshalInvoice(jsonData []byte) (models.UpdateInvoice, error) {
	var invoice models.UpdateInvoice

	err := json.Unmarshal(jsonData, &invoice)
	if err != nil {
		return models.UpdateInvoice{}, err
	}

	return invoice, nil
}
