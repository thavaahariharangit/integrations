package order

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListOrderService generates myob client code to fetch all features
//
// API: /Purchase/Order/Service
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/purchase/bill/bill_service/
func GenerateListOrderService() string {
	return ""
	//return client.GenerateList("OrderService", "OrderService", "/Purchase/Order/Service ")
}

// GenerateRetrieveOrderService generates myob client code to retrieve specific feature
//
// API: /Purchase/Order/Service
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/purchase/bill/bill_service/
func GenerateRetrieveOrderService() string {
	return ""
	//return client.GenerateRetrieve("OrderService", "/Purchase/Order/Service ")
}

// GenerateOrderServiceModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/purchase/bill/bill_service/
func GenerateOrderServiceModel() string {
	return client.GenerateModel("OrderService", "https://developer.myob.com/api/myob-business-api/v2/purchase/bill/bill_service/")
}
