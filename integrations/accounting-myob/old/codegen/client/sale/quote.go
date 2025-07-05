package sale

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListSaleQuote generates myob client code to fetch all features
//
// API: /Sale/Quote
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/sale/quote/
func GenerateListSaleQuote() string {
	return ""
	//return client.GenerateList("Quote", "Quote", "/Sale/Quote ")
}

// GenerateSaleQuoteModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/sale/quote/
func GenerateSaleQuoteModel() string {
	return client.GenerateModel("Quote", "https://developer.myob.com/api/myob-business-api/v2/sale/quote/")
}
