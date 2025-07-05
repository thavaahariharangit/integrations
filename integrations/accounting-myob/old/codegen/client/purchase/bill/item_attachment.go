package bill

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateRetrieveBillItemAttachment generates myob client code to retrieve specific feature
//
// API: /Purchase/Bill/Item/{Bill_UID}/Attachment
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/purchase/bill/bill_item/
func GenerateRetrieveBillItemAttachment() string {
	return ""
	//return client.GenerateRetrieve("BillItemAttachment", "/Purchase/Bill/Item ")
}

// GenerateBillItemAttachmentModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/purchase/bill/bill_item/
func GenerateBillItemAttachmentModel() string {
	return client.GenerateModel("BillItemAttachment", "https://developer.myob.com/api/myob-business-api/v2/purchase/bill/bill_item/")
}
