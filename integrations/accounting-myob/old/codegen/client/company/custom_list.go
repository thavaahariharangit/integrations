package company

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListCompanyCustomList generates myob client code to fetch all features
//
// API: /Company/CustomList
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/company/custom-list/
func GenerateListCompanyCustomList() string {
	return ""
	//return client.GenerateList("CompanyCustomList", "CompanyCustomList", "/Company/CustomList ")
}

// GenerateRetrieveCompanyCustomList generates myob client code to retrieve specific feature
//
// API: /Company/CustomList
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/company/custom-list/
func GenerateRetrieveCompanyCustomList() string {
	return ""
	//return client.GenerateRetrieve("CompanyCustomList", "/Company/CustomList ")
}

// GenerateCompanyCustomListModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/company/custom-list/
func GenerateCompanyCustomListModel() string {
	return client.GenerateModel("CompanyCustomList", "https://developer.myob.com/api/myob-business-api/v2/company/custom-list/")
}
