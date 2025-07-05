package ledger

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListLedgerAccount generates myob client code to fetch all features
//
// API: /GeneralLedger/Account
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/generalledger/account/
func GenerateListLedgerAccount() string {
	return ""
	//return client.GenerateList("LedgerAccount", "LedgerAccount", "/GeneralLedger/Account")
}

// GenerateRetrieveLedgerAccount generates myob client code to retrieve specific feature
//
// API: /GeneralLedger/Account
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/generalledger/account/
func GenerateRetrieveLedgerAccount() string {
	return ""
	//return client.GenerateRetrieve("LedgerAccount", "/GeneralLedger/Account")
}

// GenerateLedgerAccountModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/generalledger/account/
func GenerateLedgerAccountModel() string {
	//return ""
	return client.GenerateModel("LedgerAccount", "https://developer.myob.com/api/myob-business-api/v2/generalledger/account/")
}
