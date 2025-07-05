package client

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateRetrieveCurrentUser generates myob client code to retrieve current user
//
// API: /CurrentUser
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/current-user/
func GenerateRetrieveCurrentUser() string {
	return ""
	//return client.GenerateRetrieve("CurrentUser", "/CurrentUser")
}

// GenerateCurrentUserModel generates myob CurrentUser domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/current-user/
func GenerateCurrentUserModel() string {
	return client.GenerateModel("CurrentUser", "https://developer.myob.com/api/myob-business-api/v2/current-user/")
}
