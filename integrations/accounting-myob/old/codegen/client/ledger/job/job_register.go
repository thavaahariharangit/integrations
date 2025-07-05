package job

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListLedgerJobRegister generates myob client code to fetch all features
//
// API: /GeneralLedger/JobRegister
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/generalledger/jobregister/
func GenerateListLedgerJobRegister() string {
	return ""
	//return client.GenerateList("LedgerJobRegister", "LedgerJobRegister", "/GeneralLedger/JobRegister")
}

// GenerateRetrieveLedgerJobRegister generates myob client code to retrieve specific feature
//
// API: /GeneralLedger/JobRegister
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/generalledger/jobregister/
func GenerateRetrieveLedgerJobRegister() string {
	return ""
	//return client.GenerateRetrieve("LedgerJobRegister", "/GeneralLedger/JobRegister")
}

// GenerateLedgerJobRegisterModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/generalledger/jobregister/
func GenerateLedgerJobRegisterModel() string {
	return client.GenerateModel("LedgerJobRegister", "https://developer.myob.com/api/myob-business-api/v2/generalledger/jobregister/")
}
