package banking

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListSpendMoneyTransaction generates myob client code to fetch all features
//
// API: /Banking/SpendMoneyTxn
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/banking/spend_money/
func GenerateListSpendMoneyTransaction() string {
	return ""
	//return client.GenerateList("SpendMoneyTransaction", "SpendMoneyTransaction", "/Banking/SpendMoneyTxn")
}

// GenerateRetrieveSpendMoneyTransaction generates myob client code to retrieve specific feature
//
// API: /Banking/SpendMoneyTxn
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/banking/spend_money/
func GenerateRetrieveSpendMoneyTransaction() string {
	return ""
	//return client.GenerateRetrieve("SpendMoneyTransaction", "/Banking/SpendMoneyTxn")
}

// GenerateSpendMoneyTransactionModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/banking/spend_money/
func GenerateSpendMoneyTransactionModel() string {
	return client.GenerateModel("SpendMoneyTransaction", "https://developer.myob.com/api/myob-business-api/v2/banking/spend_money/")
}
