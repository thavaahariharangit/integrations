package inventory

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListInventoryLocation generates myob client code to fetch all features
//
// API: /Inventory/Location
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/inventory/location/
func GenerateListInventoryLocation() string {
	return ""
	//return client.GenerateList("InventoryLocation", "InventoryLocation", "/Inventory/Location")
}

// GenerateRetrieveInventoryLocation generates myob client code to retrieve specific feature
//
// API: /Inventory/Location
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/inventory/location/
func GenerateRetrieveInventoryLocation() string {
	return ""
	//return client.GenerateRetrieve("InventoryLocation", "/Inventory/Location")
}

// GenerateInventoryLocationModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/inventory/location/
func GenerateInventoryLocationModel() string {
	//return ""
	return client.GenerateModel("InventoryLocation", "https://developer.myob.com/api/myob-business-api/v2/inventory/location/")
}
