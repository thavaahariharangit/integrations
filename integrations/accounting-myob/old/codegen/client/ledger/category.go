package ledger

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListLedgerCategory generates myob client code to fetch all features
//
// API: /GeneralLedger/Category
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/generalledger/category/
func GenerateListLedgerCategory() string {
	return ""
	//return client.GenerateList("LedgerCategory", "LedgerCategory", "/GeneralLedger/Category")
}

// GenerateRetrieveLedgerCategory generates myob client code to retrieve specific feature
//
// API: /GeneralLedger/Category
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/generalledger/category/
func GenerateRetrieveLedgerCategory() string {
	return ""
	//return client.GenerateRetrieve("LedgerCategory", "/GeneralLedger/Category")
}

// GenerateLedgerCategoryModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/generalledger/category/
func GenerateLedgerCategoryModel() string {
	//return ""
	return client.GenerateModel("LedgerCategory", "https://developer.myob.com/api/myob-business-api/v2/generalledger/category/")
}
