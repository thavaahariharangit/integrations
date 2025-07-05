package client

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateRetrieveContact generates myob client code to retrieve contact
//
// API: /Contact
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/contact/
func GenerateRetrieveContact() string {
	return ""
	//return client.GenerateRetrieve("Contact", "/Contact")
}

// GenerateContactModel generates myob Contact domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/contact/
func GenerateContactModel() string {
	return client.GenerateModel("Contact", "https://developer.myob.com/api/myob-business-api/v2/contact/")
}
