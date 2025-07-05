package account

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListLedgerAccountBudget generates myob client code to fetch all features
//
// API: /GeneralLedger/AccountBudget
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/generalledger/accountbudget/
func GenerateListLedgerAccountBudget() string {
	return ""
	//return client.GenerateList("LedgerAccountBudget", "LedgerAccountBudget", "/GeneralLedger/AccountBudget")
}

// GenerateRetrieveLedgerAccountBudget generates myob client code to retrieve specific feature
//
// API: /GeneralLedger/AccountBudget
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/generalledger/accountbudget/
func GenerateRetrieveLedgerAccountBudget() string {
	return ""
	//return client.GenerateRetrieve("LedgerAccountBudget", "/GeneralLedger/AccountBudget")
}

// GenerateLedgerAccountBudgetModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/generalledger/accountbudget/
func GenerateLedgerAccountBudgetModel() string {
	//return ""
	return client.GenerateModel("LedgerAccountBudget", "https://developer.myob.com/api/myob-business-api/v2/generalledger/accountbudget/")
}
