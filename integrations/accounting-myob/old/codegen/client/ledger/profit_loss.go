package ledger

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListLedgerProfitLossDistribution generates myob client code to fetch all features
//
// API: /GeneralLedger/ProfitLossDistribution
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/generalledger/profitloss-distribution/
func GenerateListLedgerProfitLossDistribution() string {
	return ""
	//return client.GenerateList("LedgerProfitLossDistribution", "LedgerProfitLossDistribution", "/GeneralLedger/ProfitLossDistribution")
}

// GenerateRetrieveLedgerProfitLossDistribution generates myob client code to retrieve specific feature
//
// API: /GeneralLedger/ProfitLossDistribution
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/generalledger/profitloss-distribution/
func GenerateRetrieveLedgerProfitLossDistribution() string {
	return ""
	//return client.GenerateRetrieve("LedgerProfitLossDistribution", "/GeneralLedger/ProfitLossDistribution")
}

// GenerateLedgerProfitLossDistributionModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/generalledger/profitloss-distribution/
func GenerateLedgerProfitLossDistributionModel() string {
	return client.GenerateModel("LedgerProfitLossDistribution", "https://developer.myob.com/api/myob-business-api/v2/generalledger/profitloss-distribution/")
}
