package company

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListCompanyPreferences generates myob client code to fetch all features
//
// API: /Company/Preferences
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/company/preferences/
func GenerateListCompanyPreferences() string {
	return ""
	//return client.GenerateList("CompanyPreferences", "CompanyPreferences", "/Company/Preferences ")
}

// GenerateRetrieveCompanyPreferences generates myob client code to retrieve specific feature
//
// API: /Company/Preferences
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/company/preferences/
func GenerateRetrieveCompanyPreferences() string {
	return ""
	//return client.GenerateRetrieve("CompanyPreferences", "/Company/Preferences ")
}

// GenerateCompanyPreferencesModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/company/preferences/
func GenerateCompanyPreferencesModel() string {
	return client.GenerateModel("CompanyPreferences", "https://developer.myob.com/api/myob-business-api/v2/company/preferences/")
}
