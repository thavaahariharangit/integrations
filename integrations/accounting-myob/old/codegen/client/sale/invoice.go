package sale

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListSaleInvoice generates myob client code to fetch all features
//
// API: /Sale/Invoice
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/sale/invoice/
func GenerateListSaleInvoice() string {
	return ""
	//return client.GenerateList("Invoice", "Invoice", "/Sale/Invoice ")
}

// GenerateSaleInvoiceModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/sale/invoice/
func GenerateSaleInvoiceModel() string {
	return client.GenerateModel("Invoice", "https://developer.myob.com/api/myob-business-api/v2/sale/invoice/")
}
