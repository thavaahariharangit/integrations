package order

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListOrderProfessional generates myob client code to fetch all features
//
// API: /Purchase/Order/Professional
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/purchase/order/order_professional/
func GenerateListOrderProfessional() string {
	return ""
	//return client.GenerateList("OrderProfessional", "OrderProfessional", "/Purchase/Order/Professional ")
}

// GenerateRetrieveOrderProfessional generates myob client code to retrieve specific feature
//
// API: /Purchase/Order/Professional
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/purchase/order/order_professional/
func GenerateRetrieveOrderProfessional() string {
	return ""
	//return client.GenerateRetrieve("OrderProfessional", "/Purchase/Order/Professional ")
}

// GenerateOrderProfessionalModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/purchase/order/order_professional/
func GenerateOrderProfessionalModel() string {
	return client.GenerateModel("OrderProfessional", "https://developer.myob.com/api/myob-business-api/v2/purchase/order/order_professional/")
}
