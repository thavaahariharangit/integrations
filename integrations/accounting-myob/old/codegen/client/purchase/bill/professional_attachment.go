package bill

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateRetrieveBillProfessionalAttachment generates myob client code to retrieve specific feature
//
// API: /Purchase/Bill/Professional/{Bill_UID}/Attachment
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/purchase/bill/bill_professional/professional-bill-attachment/
func GenerateRetrieveBillProfessionalAttachment() string {
	return ""
	//return client.GenerateRetrieve("BillProfessionalAttachment", "/Purchase/Bill/Professional ")
}

// GenerateBillProfessionalAttachmentModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/purchase/bill/bill_professional/professional-bill-attachment/
func GenerateBillProfessionalAttachmentModel() string {
	return client.GenerateModel("BillProfessionalAttachment", "https://developer.myob.com/api/myob-business-api/v2/purchase/bill/bill_item/")
}
