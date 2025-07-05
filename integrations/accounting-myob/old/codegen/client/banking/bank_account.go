package banking

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListBankAccount generates myob client code to fetch all features
//
// API: /Banking/BankAccount
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/banking/bank-account/
func GenerateListBankAccount() string {
	return ""
	//return client.GenerateList("BankAccount", "BankAccount", "/Banking/BankAccount")
}

// GenerateRetrieveBankAccount generates myob client code to retrieve specific feature
//
// API: /Banking/BankAccount
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/banking/bank-account/
func GenerateRetrieveBankAccount() string {
	return ""
	//return client.GenerateRetrieve("BankAccount", "/Banking/BankAccount")
}

// GenerateBankAccountModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/banking/bank-account/
func GenerateBankAccountModel() string {
	return client.GenerateModel("BankAccount", "https://developer.myob.com/api/myob-business-api/v2/banking/bank-account/")
}
