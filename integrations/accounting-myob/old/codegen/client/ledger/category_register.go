package ledger

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListLedgerCategoryRegister generates myob client code to fetch all features
//
// API: /GeneralLedger/CategoryRegister
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/generalledger/categoryregister/
func GenerateListLedgerCategoryRegister() string {
	return ""
	//return client.GenerateList("LedgerCategoryRegister", "LedgerCategoryRegister", "/GeneralLedger/CategoryRegister")
}

// GenerateRetrieveLedgerCategoryRegister generates myob client code to retrieve specific feature
//
// API: /GeneralLedger/CategoryRegister
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/generalledger/categoryregister/
func GenerateRetrieveLedgerCategoryRegister() string {
	return ""
	//return client.GenerateRetrieve("LedgerCategoryRegister", "/GeneralLedger/CategoryRegister")
}

// GenerateLedgerCategoryRegisterModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/generalledger/categoryregister/
func GenerateLedgerCategoryRegisterModel() string {
	return client.GenerateModel("LedgerCategoryRegister", "https://developer.myob.com/api/myob-business-api/v2/generalledger/categoryregister/")
}
