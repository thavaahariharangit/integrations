package client

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateRetrieveCompany generates myob client code to retrieve contact
//
// API: /Company
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/company/
func GenerateRetrieveCompany() string {
	return ""
	//return client.GenerateRetrieve("Company", "/Company")
}

// GenerateCompanyModel generates myob Company domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/company/
func GenerateCompanyModel() string {
	return client.GenerateModel("Company", "https://developer.myob.com/api/myob-business-api/v2/company/")
}
