package bill

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListBillMiscellaneous generates myob client code to fetch all features
//
// API: /Purchase/Bill/Miscellaneous
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/purchase/bill/bill_item/
func GenerateListBillMiscellaneous() string {
	return ""
	//return client.GenerateList("BillMiscellaneous", "BillMiscellaneous", "/Purchase/Bill/Miscellaneous ")
}

// GenerateRetrieveBillMiscellaneous generates myob client code to retrieve specific feature
//
// API: /Purchase/Bill/Miscellaneous
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/purchase/bill/bill_item/
func GenerateRetrieveBillMiscellaneous() string {
	return ""
	//return client.GenerateRetrieve("BillMiscellaneous", "/Purchase/Bill/Miscellaneous ")
}

// GenerateBillMiscellaneousModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/purchase/bill/bill_item/
func GenerateBillMiscellaneousModel() string {
	return client.GenerateModel("BillMiscellaneous", "https://developer.myob.com/api/myob-business-api/v2/purchase/bill/bill_item/")
}
