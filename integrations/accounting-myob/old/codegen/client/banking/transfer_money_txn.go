package banking

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListTransferMoneyTransaction generates myob client code to fetch all features
//
// API: /Banking/TransferMoneyTxn
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/banking/transfer_money/
func GenerateListTransferMoneyTransaction() string {
	return ""
	//return client.GenerateList("TransferMoneyTransaction", "TransferMoneyTransaction", "/Banking/TransferMoneyTxn")
}

// GenerateRetrieveTransferMoneyTransaction generates myob client code to retrieve specific feature
//
// API: /Banking/TransferMoneyTxn
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/banking/transfer_money/
func GenerateRetrieveTransferMoneyTransaction() string {
	return ""
	//return client.GenerateRetrieve("TransferMoneyTransaction", "/Banking/TransferMoneyTxn")
}

// GenerateTransferMoneyTransactionModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/banking/transfer_money/
func GenerateTransferMoneyTransactionModel() string {
	return client.GenerateModel("TransferMoneyTransaction", "https://developer.myob.com/api/myob-business-api/v2/banking/transfer_money/")
}
