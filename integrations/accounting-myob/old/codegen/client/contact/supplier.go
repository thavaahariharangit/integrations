package contact

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListSupplierContact generates myob client code to fetch all features
//
// API: /Contact/Supplier
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/contact/supplier/
func GenerateListSupplierContact() string {
	return ""
	//return client.GenerateList("SupplierContact", "SupplierContact", "/Contact/Supplier")
}

// GenerateRetrieveSupplierContact generates myob client code to retrieve specific feature
//
// API: /Contact/Supplier
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/contact/supplier/
func GenerateRetrieveSupplierContact() string {
	return ""
	//return client.GenerateRetrieve("SupplierContact", "/Contact/Supplier")
}

// GenerateRetrieveSupplierContactPhoto generates myob client code to retrieve specific supplier contact photo
//
// API: /Contact/Supplier{contact-id}/Photo
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/contact/
func GenerateRetrieveSupplierContactPhoto() string {
	return ""
	//return client.GenerateRetrieve("Contact", "/Contact/Supplier", "/Photo")
}

// GenerateSupplierContactModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/contact/supplier/
func GenerateSupplierContactModel() string {
	return client.GenerateModelWithParent("SupplierContact", "Contact", "https://developer.myob.com/api/myob-business-api/v2/contact/supplier/")
}
