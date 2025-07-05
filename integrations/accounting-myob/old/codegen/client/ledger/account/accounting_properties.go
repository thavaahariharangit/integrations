package account

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListLedgerAccountingProperties generates myob client code to fetch all features
//
// API: /GeneralLedger/AccountingProperties
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/generalledger/accountingproperties/
func GenerateListLedgerAccountingProperties() string {
	return ""
	//return client.GenerateList("LedgerAccountingProperties", "LedgerAccountingProperties", "/GeneralLedger/AccountingProperties")
}

// GenerateRetrieveLedgerAccountingProperties generates myob client code to retrieve specific feature
//
// API: /GeneralLedger/AccountingProperties
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/generalledger/accountingproperties/
func GenerateRetrieveLedgerAccountingProperties() string {
	return ""
	//return client.GenerateRetrieve("LedgerAccountingProperties", "/GeneralLedger/AccountingProperties")
}

// GenerateLedgerAccountingPropertiesModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/generalledger/accountingproperties/
func GenerateLedgerAccountingPropertiesModel() string {
	return client.GenerateModel("LedgerAccountingProperties", "https://developer.myob.com/api/myob-business-api/v2/generalledger/accountingproperties/")
}
