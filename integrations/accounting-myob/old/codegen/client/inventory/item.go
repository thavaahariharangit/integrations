package inventory

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListInventoryItem generates myob client code to fetch all features
//
// API: /Inventory/Item
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/inventory/item/
func GenerateListInventoryItem() string {
	return ""
	//return client.GenerateList("InventoryItem", "InventoryItem", "/Inventory/Item")
}

// GenerateRetrieveInventoryItem generates myob client code to retrieve specific feature
//
// API: /Inventory/Item
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/inventory/item/
func GenerateRetrieveInventoryItem() string {
	return ""
	//return client.GenerateRetrieve("InventoryItem", "/Inventory/Item")
}

// GenerateInventoryItemModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/inventory/item/
func GenerateInventoryItemModel() string {
	return client.GenerateModel("InventoryItem", "https://developer.myob.com/api/myob-business-api/v2/inventory/item/")
}
