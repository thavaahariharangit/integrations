package contact

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListEmployeeContact generates myob client code to fetch all features
//
// API: /Contact/Employee
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/contact/employee/
func GenerateListEmployeeContact() string {
	return ""
	//return client.GenerateList("EmployeeContact", "EmployeeContact", "/Contact/Employee")
}

// GenerateRetrieveEmployeeContact generates myob client code to retrieve specific feature
//
// API: /Contact/Employee
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/contact/employee/
func GenerateRetrieveEmployeeContact() string {
	return ""
	//return client.GenerateRetrieve("EmployeeContact", "/Contact/Employee")
}

// GenerateRetrieveEmployeeContactPhoto generates myob client code to retrieve specific employee contact photo
//
// API: /Contact/Employee{contact-id}/Photo
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/contact/
func GenerateRetrieveEmployeeContactPhoto() string {
	return ""
	//return client.GenerateRetrieve("Contact", "/Contact/Employee", "/Photo")
}

// GenerateEmployeeContactModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/contact/employee/
func GenerateEmployeeContactModel() string {
	return client.GenerateModelWithParent("EmployeeContact", "Contact", "https://developer.myob.com/api/myob-business-api/v2/contact/employee/")
}
