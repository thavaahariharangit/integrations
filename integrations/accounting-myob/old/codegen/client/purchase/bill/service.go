package bill

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListBillService generates myob client code to fetch all features
//
// API: /Purchase/Bill/Service
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/purchase/bill/bill_service/
func GenerateListBillService() string {
	return ""
	//return client.GenerateList("BillService", "BillService", "/Purchase/Bill/Service ")
}

// GenerateRetrieveBillService generates myob client code to retrieve specific feature
//
// API: /Purchase/Bill/Service
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/purchase/bill/bill_service/
func GenerateRetrieveBillService() string {
	return ""
	//return client.GenerateRetrieve("BillService", "/Purchase/Bill/Service ")
}

// GenerateBillServiceModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/purchase/bill/bill_service/
func GenerateBillServiceModel() string {
	return client.GenerateModel("BillService", "https://developer.myob.com/api/myob-business-api/v2/purchase/bill/bill_service/")
}
