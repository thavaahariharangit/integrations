package payroll

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListEmploymentClassification generates myob client code to fetch all features
//
// API: /Payroll/EmploymentClassification
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/payroll/employment-classification/
func GenerateListEmploymentClassification() string {
	return ""
	//return client.GenerateList("EmploymentClassification", "EmploymentClassification", "/Payroll/EmploymentClassification ")
}

// GenerateRetrieveEmploymentClassification generates myob client code to retrieve specific feature
//
// API: /Payroll/EmploymentClassification
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/payroll/employment-classification/
func GenerateRetrieveEmploymentClassification() string {
	return ""
	//return client.GenerateRetrieve("EmploymentClassification", "/Payroll/EmploymentClassification ")
}

// GenerateEmploymentClassificationModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/payroll/employment-classification/
func GenerateEmploymentClassificationModel() string {
	//return ""
	return client.GenerateModel("EmploymentClassification", "https://developer.myob.com/api/myob-business-api/v2/payroll/employment-classification/")
}
