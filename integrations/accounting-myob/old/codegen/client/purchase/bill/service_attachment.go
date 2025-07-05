package bill

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateRetrieveBillServiceAttachment generates myob client code to retrieve specific feature
//
// API: /Purchase/Bill/Service/{Bill_UID}/Attachment
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/purchase/bill/bill_service/service-bill-attachment/
func GenerateRetrieveBillServiceAttachment() string {
	return ""
	//return client.GenerateRetrieve("BillServiceAttachment", "/Purchase/Bill/Service ")
}

// GenerateBillServiceAttachmentModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/purchase/bill/bill_service/service-bill-attachment/
func GenerateBillServiceAttachmentModel() string {
	return client.GenerateModel("BillServiceAttachment", "https://developer.myob.com/api/myob-business-api/v2/purchase/bill/bill_item/")
}
