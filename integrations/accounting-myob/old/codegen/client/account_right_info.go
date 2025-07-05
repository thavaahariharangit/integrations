package client

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateRetrieveInfo generates freshbooks client code to retrieve specific payment
//
// API: /accountright/Info
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/info/
func GenerateRetrieveInfo() string {
	return ""
	//return client.GenerateRetrieve("Info", "/accountright/Info")
}

// GenerateInfoModel generates freshbooks Info domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/info/
func GenerateInfoModel() string {
	return client.GenerateModel("Info", "https://developer.myob.com/api/myob-business-api/v2/info/")
}
