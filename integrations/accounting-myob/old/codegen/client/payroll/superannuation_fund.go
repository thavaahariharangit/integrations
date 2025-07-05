package payroll

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListSuperannuationFund generates myob client code to fetch all features
//
// API: /Payroll/SuperannuationFund
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/payroll/superannuationfund/
func GenerateListSuperannuationFund() string {
	return ""
	//return client.GenerateList("SuperannuationFund", "SuperannuationFund", "/Payroll/SuperannuationFund ")
}

// GenerateRetrieveSuperannuationFund generates myob client code to retrieve specific feature
//
// API: /Payroll/SuperannuationFund
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/payroll/superannuationfund/
func GenerateRetrieveSuperannuationFund() string {
	return ""
	//return client.GenerateRetrieve("SuperannuationFund", "/Payroll/SuperannuationFund ")
}

// GenerateSuperannuationFundModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/payroll/superannuationfund/
func GenerateSuperannuationFundModel() string {
	return client.GenerateModel("SuperannuationFund", "https://developer.myob.com/api/myob-business-api/v2/payroll/superannuationfund/")
}
