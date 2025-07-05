package employee

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListEmployeePayrollDetails generates myob client code to fetch all features
//
// API: /Contact/EmployeePaymentDetails
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/contact/employee-payment-details/
func GenerateListEmployeePayrollDetails() string {
	return ""
	//return client.GenerateList("EmployeePayrollDetails", "EmployeePayrollDetails", "/Contact/Customer")
}

// GenerateRetrieveEmployeePayrollDetails generates myob client code to retrieve specific feature
//
// API: /Contact/EmployeePaymentDetails
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/contact/employee-payment-details/
func GenerateRetrieveEmployeePayrollDetails() string {
	return ""
	//return client.GenerateRetrieve("EmployeePayrollDetails", "/Contact/Customer")
}

// GenerateEmployeePayrollDetailsModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/contact/employee-payment-details/
func GenerateEmployeePayrollDetailsModel() string {
	return client.GenerateModelWithParent("EmployeePayrollDetails", "Contact", "https://developer.myob.com/api/myob-business-api/v2/contact/employee-payroll-details/")
}
