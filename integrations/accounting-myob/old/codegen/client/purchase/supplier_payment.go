package purchase

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListSupplierPayment generates myob client code to fetch all features
//
// API: /Purchase/SupplierPayment
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/purchase/supplierpayment/
func GenerateListSupplierPayment() string {
	return ""
	//return client.GenerateList("SupplierPayment", "SupplierPayment", "/Purchase/SupplierPayment ")
}

// GenerateRetrieveSupplierPayment generates myob client code to retrieve specific feature
//
// API: /Purchase/SupplierPayment
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/purchase/supplierpayment/
func GenerateRetrieveSupplierPayment() string {
	return ""
	//return client.GenerateRetrieve("SupplierPayment", "/Purchase/SupplierPayment ")
}

// GenerateSupplierPaymentModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/purchase/supplierpayment/
func GenerateSupplierPaymentModel() string {
	return client.GenerateModel("SupplierPayment", "https://developer.myob.com/api/myob-business-api/v2/purchase/supplierpayment/")
}
