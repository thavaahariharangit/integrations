package job

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListLedgerJobBudget generates myob client code to fetch all features
//
// API: /GeneralLedger/JobBudget
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/generalledger/jobbudget/
func GenerateListLedgerJobBudget() string {
	return ""
	//return client.GenerateList("LedgerJobBudget", "LedgerJobBudget", "/GeneralLedger/JobBudget")
}

// GenerateRetrieveLedgerJobBudget generates myob client code to retrieve specific feature
//
// API: /GeneralLedger/JobBudget
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/generalledger/jobbudget/
func GenerateRetrieveLedgerJobBudget() string {
	return ""
	//return client.GenerateRetrieve("LedgerJobBudget", "/GeneralLedger/JobBudget")
}

// GenerateLedgerJobBudgetModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/generalledger/jobbudget/
func GenerateLedgerJobBudgetModel() string {
	//return ""
	return client.GenerateModel("LedgerJobBudget", "https://developer.myob.com/api/myob-business-api/v2/generalledger/jobbudget/")
}
