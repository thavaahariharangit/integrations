package payroll

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListPayrollCategory generates myob client code to fetch all features
//
// API: /Payroll/PayrollCategory
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/payroll/payroll-category/
func GenerateListPayrollCategory() string {
	return ""
	//return client.GenerateList("PayrollCategory", "PayrollCategory", "/Payroll/PayrollCategory ")
}

// GenerateRetrievePayrollCategory generates myob client code to retrieve specific feature
//
// API: /Payroll/PayrollCategory
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/payroll/payroll-category/
func GenerateRetrievePayrollCategory() string {
	return ""
	//return client.GenerateRetrieve("PayrollCategory", "/Payroll/PayrollCategory ")
}

// GeneratePayrollCategoryModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/payroll/payroll-category/
func GeneratePayrollCategoryModel() string {
	return client.GenerateModel("PayrollCategory", "https://developer.myob.com/api/myob-business-api/v2/payroll/payroll-category/")
}
