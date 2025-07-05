package sale

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListSaleCreditRefund generates myob client code to fetch all features
//
// API: /Sale/CreditRefund
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/purchase/bill/
func GenerateListSaleCreditRefund() string {
	return ""
	//return client.GenerateList("CreditRefund", "CreditRefund", "/Sale/CreditRefund ")
}

// GenerateSaleCreditRefundModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/purchase/bill/
func GenerateSaleCreditRefundModel() string {
	return client.GenerateModel("CreditRefund", "https://developer.myob.com/api/myob-business-api/v2/purchase/bill/")
}
