package ledger

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListLedgerGeneralJournal generates myob client code to fetch all features
//
// API: /GeneralLedger/GeneralJournal
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/generalledger/generaljournal/
func GenerateListLedgerGeneralJournal() string {
	return ""
	//return client.GenerateList("LedgerGeneralJournal", "LedgerGeneralJournal", "/GeneralLedger/GeneralJournal")
}

// GenerateRetrieveLedgerGeneralJournal generates myob client code to retrieve specific feature
//
// API: /GeneralLedger/GeneralJournal
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/generalledger/generaljournal/
func GenerateRetrieveLedgerGeneralJournal() string {
	return ""
	//return client.GenerateRetrieve("LedgerGeneralJournal", "/GeneralLedger/GeneralJournal")
}

// GenerateLedgerGeneralJournalModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/generalledger/generaljournal/
func GenerateLedgerGeneralJournalModel() string {
	return client.GenerateModel("LedgerGeneralJournal", "https://developer.myob.com/api/myob-business-api/v2/generalledger/generaljournal/")
}
