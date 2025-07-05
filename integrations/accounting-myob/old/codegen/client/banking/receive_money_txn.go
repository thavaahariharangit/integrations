package banking

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListMoneyTransaction generates myob client code to fetch all features
//
// API: /Banking/ReceiveMoneyTxn
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/banking/receive_money/
func GenerateListMoneyTransaction() string {
	return ""
	//return client.GenerateList("ReceiveMoneyTransaction", "ReceiveMoneyTransaction", "/Banking/ReceiveMoneyTxn")
}

// GenerateReceiveMoneyTransaction generates myob client code to retrieve specific feature
//
// API: /Banking/ReceiveMoneyTxn
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/banking/receive_money/
func GenerateReceiveMoneyTransaction() string {
	return ""
	//return client.GenerateRetrieve("ReceiveMoneyTransaction", "/Banking/ReceiveMoneyTxn")
}

// GenerateReceiveMoneyTransactionModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/banking/receive_money/
func GenerateReceiveMoneyTransactionModel() string {
	return client.GenerateModel("ReceiveMoneyTransaction", "https://developer.myob.com/api/myob-business-api/v2/banking/receive_money/")
}
