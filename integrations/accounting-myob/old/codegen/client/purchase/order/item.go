package order

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListOrderItem generates myob client code to fetch all features
//
// API: /Purchase/Order/Item
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/purchase/order/order_item/
func GenerateListOrderItem() string {
	return ""
	//return client.GenerateList("OrderItem", "OrderItem", "/Purchase/Order/Item ")
}

// GenerateRetrieveOrderItem generates myob client code to retrieve specific feature
//
// API: /Purchase/Order/Item
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/purchase/order/order_item/
func GenerateRetrieveOrderItem() string {
	return ""
	//return client.GenerateRetrieve("OrderItem", "/Purchase/Order/Item ")
}

// GenerateOrderItemModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/purchase/order/order_item/
func GenerateOrderItemModel() string {
	return client.GenerateModel("OrderItem", "https://developer.myob.com/api/myob-business-api/v2/purchase/order/order_item/")
}
