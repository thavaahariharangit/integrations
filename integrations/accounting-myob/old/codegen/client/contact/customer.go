package contact

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListCustomerContact generates myob client code to fetch all features
//
// API: /Contact/Customer
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/contact/customer/
func GenerateListCustomerContact() string {
	return ""
	//return client.GenerateList("CustomerContact", "CustomerContact", "/Contact/Customer")
}

// GenerateRetrieveCustomerContact generates myob client code to retrieve specific feature
//
// API: /Contact/Customer
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/contact/customer/
func GenerateRetrieveCustomerContact() string {
	return ""
	//return client.GenerateRetrieve("CustomerContact", "/Contact/Customer")
}

// GenerateRetrieveCustomerContactPhoto generates myob client code to retrieve specific customer contact photo
//
// API: /Contact/Customer{contact-id}/Photo
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/contact/
func GenerateRetrieveCustomerContactPhoto() string {
	return ""
	//return client.GenerateRetrieve("Contact", "/Contact/Customer", "/Photo")
}

// GenerateCustomerContactModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/contact/customer/
func GenerateCustomerContactModel() string {
	return client.GenerateModelWithParent("CustomerContact", "Contact", "https://developer.myob.com/api/myob-business-api/v2/contact/customer/")

}
