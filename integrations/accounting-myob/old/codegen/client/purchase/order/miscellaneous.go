package order

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListOrderMiscellaneous generates myob client code to fetch all features
//
// API: /Purchase/Order/Miscellaneous
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/purchase/order/order_miscellaneous/
func GenerateListOrderMiscellaneous() string {
	return ""
	//return client.GenerateList("OrderMiscellaneous", "OrderMiscellaneous", "/Purchase/Order/Miscellaneous ")
}

// GenerateRetrieveOrderMiscellaneous generates myob client code to retrieve specific feature
//
// API: /Purchase/Order/Miscellaneous
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/purchase/order/order_miscellaneous/
func GenerateRetrieveOrderMiscellaneous() string {
	return ""
	//return client.GenerateRetrieve("OrderMiscellaneous", "/Purchase/Order/Miscellaneous ")
}

// GenerateOrderMiscellaneousModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/purchase/order/order_miscellaneous/
func GenerateOrderMiscellaneousModel() string {
	return client.GenerateModel("OrderMiscellaneous", "https://developer.myob.com/api/myob-business-api/v2/purchase/order/order_miscellaneous/")
}
