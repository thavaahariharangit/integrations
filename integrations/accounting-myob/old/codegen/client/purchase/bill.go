package purchase

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListPurchaseBill generates myob client code to fetch all features
//
// API: /Purchase/Bill
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/purchase/bill/
func GenerateListPurchaseBill() string {
	return ""
	//return client.GenerateList("Bill", "Bill", "/Purchase/Bill ")
}

// GenerateRetrievePurchaseBill generates myob client code to retrieve specific feature
//
// API: /Purchase/Bill
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/purchase/bill/
func GenerateRetrievePurchaseBill() string {
	return ""
	//return client.GenerateRetrieve("Bill", "/Purchase/Bill ")
}

// GeneratePurchaseBillModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/purchase/bill/
func GeneratePurchaseBillModel() string {
	return client.GenerateModel("Bill", "https://developer.myob.com/api/myob-business-api/v2/purchase/bill/")
}
