package purchase

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListOrder generates myob client code to fetch all features
//
// API: /Purchase/Order
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/purchase/debit_settlement/
func GenerateListOrder() string {
	return ""
	//return client.GenerateList("Order", "Order", "/Purchase/Order ")
}

// GenerateRetrieveOrder generates myob client code to retrieve specific feature
//
// API: /Purchase/Order
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/purchase/debit_settlement/
func GenerateRetrieveOrder() string {
	return ""
	//return client.GenerateRetrieve("Order", "/Purchase/Order ")
}

// GenerateOrderModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/purchase/debit_settlement/
func GenerateOrderModel() string {
	return client.GenerateModel("Order", "https://developer.myob.com/api/myob-business-api/v2/purchase/debit_settlement/")
}
