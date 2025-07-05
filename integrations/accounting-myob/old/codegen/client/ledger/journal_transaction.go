package ledger

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListLedgerJournalTransaction generates myob client code to fetch all features
//
// API: /GeneralLedger/JournalTransaction
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/generalledger/journaltransaction/
func GenerateListLedgerJournalTransaction() string {
	return ""
	//return client.GenerateList("LedgerJournalTransaction", "LedgerJournalTransaction", "/GeneralLedger/JournalTransaction")
}

// GenerateRetrieveLedgerJournalTransaction generates myob client code to retrieve specific feature
//
// API: /GeneralLedger/JournalTransaction
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/generalledger/journaltransaction/
func GenerateRetrieveLedgerJournalTransaction() string {
	return ""
	//return client.GenerateRetrieve("LedgerJournalTransaction", "/GeneralLedger/JournalTransaction")
}

// GenerateLedgerJournalTransactionModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/generalledger/journaltransaction/
func GenerateLedgerJournalTransactionModel() string {
	return client.GenerateModel("LedgerJournalTransaction", "https://developer.myob.com/api/myob-business-api/v2/generalledger/journaltransaction/")
}
