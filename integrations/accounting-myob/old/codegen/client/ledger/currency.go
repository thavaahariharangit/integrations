package ledger

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListLedgerCurrency generates myob client code to fetch all features
//
// API: /GeneralLedger/Currency
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/generalledger/currency/
func GenerateListLedgerCurrency() string {
	return ""
	//return client.GenerateList("LedgerCurrency", "LedgerCurrency", "/GeneralLedger/Currency")
}

// GenerateRetrieveLedgerCurrency generates myob client code to retrieve specific feature
//
// API: /GeneralLedger/Currency
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/generalledger/currency/
func GenerateRetrieveLedgerCurrency() string {
	return ""
	//return client.GenerateRetrieve("LedgerCurrency", "/GeneralLedger/Currency")
}

// GenerateLedgerCurrencyModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/generalledger/currency/
func GenerateLedgerCurrencyModel() string {
	return client.GenerateModel("LedgerCurrency", "https://developer.myob.com/api/myob-business-api/v2/generalledger/currency/")
}
