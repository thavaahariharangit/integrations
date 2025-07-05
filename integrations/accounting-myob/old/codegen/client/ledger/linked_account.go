package ledger

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListLedgerLinkedAccount generates myob client code to fetch all features
//
// API: /GeneralLedger/LinkedAccount
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/generalledger/linkedaccount/
func GenerateListLedgerLinkedAccount() string {
	return ""
	//return client.GenerateList("LedgerLinkedAccount", "LedgerLinkedAccount", "/GeneralLedger/LinkedAccount")
}

// GenerateRetrieveLedgerLinkedAccount generates myob client code to retrieve specific feature
//
// API: /GeneralLedger/LinkedAccount
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/generalledger/linkedaccount/
func GenerateRetrieveLedgerLinkedAccount() string {
	return ""
	//return client.GenerateRetrieve("LedgerLinkedAccount", "/GeneralLedger/LinkedAccount")
}

// GenerateLedgerLinkedAccountModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/generalledger/linkedaccount/
func GenerateLedgerLinkedAccountModel() string {
	return client.GenerateModel("LedgerLinkedAccount", "https://developer.myob.com/api/myob-business-api/v2/generalledger/linkedaccount/")
}
