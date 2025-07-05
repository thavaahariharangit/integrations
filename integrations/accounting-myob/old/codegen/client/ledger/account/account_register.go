package account

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListLedgerAccountRegister generates myob client code to fetch all features
//
// API: /GeneralLedger/AccountRegister
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/generalledger/accountregister/
func GenerateListLedgerAccountRegister() string {
	return ""
	//return client.GenerateList("LedgerAccountRegister", "LedgerAccountRegister", "/GeneralLedger/AccountRegister")
}

// GenerateRetrieveLedgerAccountRegister generates myob client code to retrieve specific feature
//
// API: /GeneralLedger/AccountRegister
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/generalledger/accountregister/
func GenerateRetrieveLedgerAccountRegister() string {
	return ""
	//return client.GenerateRetrieve("LedgerAccountRegister", "/GeneralLedger/AccountRegister")
}

// GenerateLedgerAccountRegisterModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/generalledger/accountregister/
func GenerateLedgerAccountRegisterModel() string {
	//return ""
	return client.GenerateModel("LedgerAccountRegister", "https://developer.myob.com/api/myob-business-api/v2/generalledger/accountregister/")
}
