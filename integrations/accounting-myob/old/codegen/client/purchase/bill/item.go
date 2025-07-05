package bill

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListBillItem generates myob client code to fetch all features
//
// API: /Purchase/Bill/Item
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/purchase/bill/bill_item/
func GenerateListBillItem() string {
	return ""
	//return client.GenerateList("BillItem", "BillItem", "/Purchase/Bill/Item ")
}

// GenerateRetrieveBillItem generates myob client code to retrieve specific feature
//
// API: /Purchase/Bill/Item
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/purchase/bill/bill_item/
func GenerateRetrieveBillItem() string {
	return ""
	//return client.GenerateRetrieve("BillItem", "/Purchase/Bill/Item ")
}

// GenerateBillItemModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/purchase/bill/bill_item/
func GenerateBillItemModel() string {
	return client.GenerateModel("BillItem", "https://developer.myob.com/api/myob-business-api/v2/purchase/bill/bill_item/")
}
