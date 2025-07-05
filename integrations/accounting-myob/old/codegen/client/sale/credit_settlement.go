package sale

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListSaleCreditSettlement generates myob client code to fetch all features
//
// API: /Sale/CreditSettlement
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/sale/credit_settlement/
func GenerateListSaleCreditSettlement() string {
	return ""
	//return client.GenerateList("CreditSettlement", "CreditSettlement", "/Sale/CreditSettlement ")
}

// GenerateSaleCreditSettlementModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/sale/credit_settlement/
func GenerateSaleCreditSettlementModel() string {
	return client.GenerateModel("CreditSettlement", "https://developer.myob.com/api/myob-business-api/v2/sale/credit_settlement/")
}
