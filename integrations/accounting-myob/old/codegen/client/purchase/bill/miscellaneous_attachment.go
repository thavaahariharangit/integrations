package bill

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateRetrieveBillMiscellaneousAttachment generates myob client code to retrieve specific feature
//
// API: /Purchase/Bill/Miscellaneous/{Bill_UID}/Attachment
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/purchase/bill/bill_miscellaneous/miscellaneous-bill-attachment/
func GenerateRetrieveBillMiscellaneousAttachment() string {
	return ""
	//return client.GenerateRetrieve("BillMiscellaneousAttachment", "/Purchase/Bill/Miscellaneous ")
}

// GenerateBillMiscellaneousAttachmentModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/purchase/bill/bill_miscellaneous/miscellaneous-bill-attachment/
func GenerateBillMiscellaneousAttachmentModel() string {
	return client.GenerateModel("BillMiscellaneousAttachment", "https://developer.myob.com/api/myob-business-api/v2/purchase/bill/bill_item/")
}
