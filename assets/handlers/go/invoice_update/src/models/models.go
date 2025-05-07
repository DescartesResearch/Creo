package models

type Address struct {
	FirstName string `json:"first_name" validate:"required,min=2,max=64"`
	LastName  string `json:"last_name" validate:"required,min=2,max=64"`
	Street    string `json:"street" validate:"required,min=2,max=128"`
	Number    int    `json:"number" validate:"required,gt=0"`
	ZipCode   int    `json:"zip_code" validate:"required"`
	City      string `json:"city" validate:"required"`
	Country   string `json:"country" validate:"required"`
}

type Item struct {
	PriceInCents int    `json:"price_in_cents" validate:"required,gt=0"`
	Name         string `json:"name" validate:"required,min=1,max=128"`
}

type OrderItem struct {
	Item     Item `json:"item" validate:"required"`
	Quantity int  `json:"quantity" validate:"required,gt=0"`
}

func (oi *OrderItem) TotalAmountInCents() int {
	return oi.Item.PriceInCents * oi.Quantity
}

type UpdateInvoice struct {
	Items           []OrderItem `json:"items,omitempty"`
	BillingAddress  *Address    `json:"billing_address,omitempty"`
	ShippingAddress *Address    `json:"shipping_address,omitempty"`
	TaxRate         *float64    `json:"tax_rate,omitempty"`
	ExtraInfo       *string     `json:"extra_info,omitempty"`
	Status          *string     `json:"status,omitempty"`
}

func (ui *UpdateInvoice) Pay() {
	paidStatus := "PAID"
	ui.Status = &paidStatus
}
