package contact

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListPersonalContact generates myob client code to fetch all features
//
// API: /Contact/Personal
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/contact/personal/
func GenerateListPersonalContact() string {
	return ""
	//return client.GenerateList("PersonalContact", "PersonalContact", "/Contact/Personal")
}

// GenerateRetrievePersonalContact generates myob client code to retrieve specific feature
//
// API: /Contact/Personal
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/contact/personal/
func GenerateRetrievePersonalContact() string {
	return ""
	//return client.GenerateRetrieve("PersonalContact", "/Contact/Personal")
}

// GenerateRetrievePersonalContactPhoto generates myob client code to retrieve specific personal contact photo
//
// API: /Contact/Personal{contact-id}/Photo
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/contact/
func GenerateRetrievePersonalContactPhoto() string {
	return ""
	//return client.GenerateRetrieve("Contact", "/Contact/Personal", "/Photo")
}

// GeneratePersonalContactModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/contact/personal/
func GeneratePersonalContactModel() string {
	return client.GenerateModelWithParent("PersonalContact", "Contact", "https://developer.myob.com/api/myob-business-api/v2/contact/personal/")
}
