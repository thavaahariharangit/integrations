package time_billing

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListTimeBillingActivitySlip generates myob client code to fetch all features
//
// API: /TimeBilling/ActivitySlip
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/timebilling/activity-slip/
func GenerateListTimeBillingActivitySlip() string {
	return ""
	//return client.GenerateList("Quote", "Quote", "/Purchase/Quote ")
}

// GenerateTimeBillingActivitySlipModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/timebilling/activity-slip/
func GenerateTimeBillingActivitySlipModel() string {
	return client.GenerateModel("TimeBillingActivitySlip", "https://developer.myob.com/api/myob-business-api/v2/timebilling/activity-slip/")
}
