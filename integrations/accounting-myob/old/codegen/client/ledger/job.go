package ledger

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListLedgerJob generates myob client code to fetch all features
//
// API: /GeneralLedger/Job
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/generalledger/job/
func GenerateListLedgerJob() string {
	return ""
	//return client.GenerateList("LedgerJob", "LedgerJob", "/GeneralLedger/Job")
}

// GenerateRetrieveLedgerJob generates myob client code to retrieve specific feature
//
// API: /GeneralLedger/Job
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/generalledger/job/
func GenerateRetrieveLedgerJob() string {
	return ""
	//return client.GenerateRetrieve("LedgerJob", "/GeneralLedger/Job")
}

// GenerateLedgerJobModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/generalledger/job/
func GenerateLedgerJobModel() string {
	//return ""
	return client.GenerateModel("LedgerJob", "https://developer.myob.com/api/myob-business-api/v2/generalledger/job/")
}
