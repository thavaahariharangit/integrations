package time_billing

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListTimeBillingActivity generates myob client code to fetch all features
//
// API: /TimeBilling/Activity
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/timebilling/activity/
func GenerateListTimeBillingActivity() string {
	return ""
	//return client.GenerateList("Quote", "Quote", "/Purchase/Quote ")
}

// GenerateTimeBillingActivityModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/timebilling/activity/
func GenerateTimeBillingActivityModel() string {
	return client.GenerateModel("TimeBillingActivity", "https://developer.myob.com/api/myob-business-api/v2/timebilling/activity/")
}
