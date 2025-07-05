package inventory

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListInventoryBuild generates myob client code to fetch all features
//
// API: /Inventory/Build
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/inventory/build/
func GenerateListInventoryBuild() string {
	return ""
	//return client.GenerateList("InventoryBuild", "InventoryBuild", "/Inventory/Build")
}

// GenerateRetrieveInventoryBuild generates myob client code to retrieve specific feature
//
// API: /Inventory/Build
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/inventory/build/
func GenerateRetrieveInventoryBuild() string {
	return ""
	//return client.GenerateRetrieve("InventoryBuild", "/Inventory/Build")
}

// GenerateInventoryBuildModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/inventory/build/
func GenerateInventoryBuildModel() string {
	return client.GenerateModel("InventoryBuild", "https://developer.myob.com/api/myob-business-api/v2/inventory/build/")
}
