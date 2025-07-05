package banking

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListStatement generates myob client code to fetch all features
//
// API: /Banking/Statement
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/banking/statement/
func GenerateListStatement() string {
	return ""
	//return client.GenerateList("Statement", "Statement", "/Banking/Statement")
}

// GenerateRetrieveStatement generates myob client code to retrieve specific feature
//
// API: /Banking/Statement
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/banking/statement/
func GenerateRetrieveStatement() string {
	return ""
	//return client.GenerateRetrieve("Statement", "/Banking/Statement")
}

// GenerateStatementModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/banking/statement/
func GenerateStatementModel() string {
	return client.GenerateModel("Statement", "https://developer.myob.com/api/myob-business-api/v2/banking/statement/")
}
