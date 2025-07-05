package sale

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListSaleOrder generates myob client code to fetch all features
//
// API: /Sale/Order
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/sale/order/
func GenerateListSaleOrder() string {
	return ""
	//return client.GenerateList("Order", "Order", "/Sale/Order ")
}

// GenerateSaleOrderModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/sale/order/
func GenerateSaleOrderModel() string {
	return client.GenerateModel("Order", "https://developer.myob.com/api/myob-business-api/v2/sale/order/")
}
