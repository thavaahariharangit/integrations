package sale

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListSaleCustomerPayment generates myob client code to fetch all features
//
// API: /Sale/CustomerPayment
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/sale/customerpayment/
func GenerateListSaleCustomerPayment() string {
	return ""
	//return client.GenerateList("CustomerPayment", "CustomerPayment", "/Sale/CustomerPayment ")
}

// GenerateSaleCustomerPaymentModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/sale/customerpayment/
func GenerateSaleCustomerPaymentModel() string {
	return client.GenerateModel("CustomerPayment", "https://developer.myob.com/api/myob-business-api/v2/sale/customerpayment/")
}
