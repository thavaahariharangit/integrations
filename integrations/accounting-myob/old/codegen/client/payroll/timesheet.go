package payroll

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListTimesheet generates myob client code to fetch all features
//
// API: /Payroll/Timesheet
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/payroll/timesheet/
func GenerateListTimesheet() string {
	return ""
	//return client.GenerateList("Timesheet", "Timesheet", "/Payroll/Timesheet ")
}

// GenerateRetrieveTimesheet generates myob client code to retrieve specific feature
//
// API: /Payroll/Timesheet
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/payroll/timesheet/
func GenerateRetrieveTimesheet() string {
	return ""
	//return client.GenerateRetrieve("Timesheet", "/Payroll/Timesheet ")
}

// GenerateTimesheetModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/payroll/timesheet/
func GenerateTimesheetModel() string {
	return client.GenerateModel("Timesheet", "https://developer.myob.com/api/myob-business-api/v2/payroll/timesheet/")
}
