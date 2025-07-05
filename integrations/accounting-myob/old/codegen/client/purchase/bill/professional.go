package bill

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListBillProfessional generates myob client code to fetch all features
//
// API: /Purchase/Bill/Professional
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/purchase/bill/bill_item/
func GenerateListBillProfessional() string {
	return ""
	//return client.GenerateList("BillProfessional", "BillProfessional", "/Purchase/Bill/Professional ")
}

// GenerateRetrieveBillProfessional generates myob client code to retrieve specific feature
//
// API: /Purchase/Bill/Professional
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/purchase/bill/bill_item/
func GenerateRetrieveBillProfessional() string {
	return ""
	//return client.GenerateRetrieve("BillProfessional", "/Purchase/Bill/Professional ")
}

// GenerateBillProfessionalModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/purchase/bill/bill_item/
func GenerateBillProfessionalModel() string {
	return client.GenerateModel("BillProfessional", "https://developer.myob.com/api/myob-business-api/v2/purchase/bill/bill_item/")
}
