package purchase

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListDebitRefund generates myob client code to fetch all features
//
// API: /Purchase/DebitRefund
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/purchase/debit_refund/
func GenerateListDebitRefund() string {
	return ""
	//return client.GenerateList("DebitRefund", "DebitRefund", "/Purchase/DebitRefund ")
}

// GenerateRetrieveDebitRefund generates myob client code to retrieve specific feature
//
// API: /Purchase/DebitRefund
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/purchase/debit_refund/
func GenerateRetrieveDebitRefund() string {
	return ""
	//return client.GenerateRetrieve("DebitRefund", "/Purchase/DebitRefund ")
}

// GenerateDebitRefundModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/purchase/debit_refund/
func GenerateDebitRefundModel() string {
	return client.GenerateModel("DebitRefund", "https://developer.myob.com/api/myob-business-api/v2/purchase/debit_refund/")
}
