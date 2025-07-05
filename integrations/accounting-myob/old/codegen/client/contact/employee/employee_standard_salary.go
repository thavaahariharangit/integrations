package employee

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListEmployeeStandardSalary generates myob client code to fetch all features
//
// API: /Contact/EmployeePaymentDetails
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/contact/employee-standard-pay/
func GenerateListEmployeeStandardSalary() string {
	return ""
	//return client.GenerateList("EmployeeStandardSalary", "EmployeeStandardSalary", "/Contact/Customer")
}

// GenerateRetrieveEmployeeStandardSalary generates myob client code to retrieve specific feature
//
// API: /Contact/EmployeePaymentDetails
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/contact/employee-standard-pay/
func GenerateRetrieveEmployeeStandardSalary() string {
	return ""
	//return client.GenerateRetrieve("EmployeeStandardSalary", "/Contact/Customer")
}

// GenerateEmployeeStandardSalaryModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/contact/employee-standard-pay/
func GenerateEmployeeStandardSalaryModel() string {
	return client.GenerateModelWithParent("EmployeePayrollDetails", "Contact", "https://developer.myob.com/api/myob-business-api/v2/contact/employee-standard-pay/")
}
