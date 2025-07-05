package employee

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListEmployeePaymentDetails generates myob client code to fetch all features
//
// API: /Contact/EmployeePayrollDetails
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/contact/employee-payment-details/
func GenerateListEmployeePaymentDetails() string {
	return ""
	//return client.GenerateList("EmployeePaymentDetails", "EmployeePaymentDetails", "/Contact/Customer")
}

// GenerateRetrieveEmployeePaymentDetails generates myob client code to retrieve specific feature
//
// API: /Contact/EmployeePayrollDetails
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/contact/employee-payment-details/
func GenerateRetrieveEmployeePaymentDetails() string {
	return ""
	//return client.GenerateRetrieve("EmployeePaymentDetails", "/Contact/Customer")
}

// GenerateEmployeePaymentDetailsModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/contact/employee-payment-details/
func GenerateEmployeePaymentDetailsModel() string {
	return client.GenerateModelWithParent("EmployeePaymentDetails", "Contact", "https://developer.myob.com/api/myob-business-api/v2/contact/employee-payment-details/")
}
