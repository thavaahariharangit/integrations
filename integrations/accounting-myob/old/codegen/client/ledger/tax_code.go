package ledger

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListLedgerTaxCode generates myob client code to fetch all features
//
// API: /GeneralLedger/TaxCode
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/generalledger/taxcode/
func GenerateListLedgerTaxCode() string {
	return ""
	//return client.GenerateList("LedgerTaxCode", "LedgerTaxCode", "/GeneralLedger/TaxCode")
}

// GenerateRetrieveLedgerTaxCode generates myob client code to retrieve specific feature
//
// API: /GeneralLedger/TaxCode
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/generalledger/taxcode/
func GenerateRetrieveLedgerTaxCode() string {
	return ""
	//return client.GenerateRetrieve("LedgerTaxCode", "/GeneralLedger/TaxCode")
}

// GenerateLedgerTaxCodeModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/generalledger/taxcode/
func GenerateLedgerTaxCodeModel() string {
	//return ""
	return client.GenerateModel("LedgerTaxCode", "https://developer.myob.com/api/myob-business-api/v2/generalledger/taxcode/")
}
