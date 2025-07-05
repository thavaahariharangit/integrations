package client

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateRetrieveCompanyFiles generates myob client code to retrieve company files
//
// API: /accountright
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/company-files/
func GenerateRetrieveCompanyFiles() string {
	return ""
	//return client.GenerateRetrieve("CompanyFiles", "/accountright")
}

// GenerateCompanyFilesModel generates myob CompanyFiles domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/company-files/
func GenerateCompanyFilesModel() string {
	return client.GenerateModel("CompanyFiles", "https://developer.myob.com/api/myob-business-api/v2/company-files/")
}
