package purchase

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListDebitSettlement generates myob client code to fetch all features
//
// API: /Purchase/DebitSettlement
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/purchase/debit_refund/
func GenerateListDebitSettlement() string {
	return ""
	//return client.GenerateList("DebitSettlement", "DebitSettlement", "/Purchase/DebitSettlement ")
}

// GenerateRetrieveDebitSettlement generates myob client code to retrieve specific feature
//
// API: /Purchase/DebitSettlement
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/purchase/debit_refund/
func GenerateRetrieveDebitSettlement() string {
	return ""
	//return client.GenerateRetrieve("DebitSettlement", "/Purchase/DebitSettlement ")
}

// GenerateDebitSettlementModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/purchase/debit_refund/
func GenerateDebitSettlementModel() string {
	return client.GenerateModel("DebitSettlement", "https://developer.myob.com/api/myob-business-api/v2/purchase/debit_refund/")
}
