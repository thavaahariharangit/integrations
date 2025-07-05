package inventory

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListInventoryAdjustment generates myob client code to fetch all features
//
// API: /Inventory/Adjustment
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/inventory/adjustment/
func GenerateListInventoryAdjustment() string {
	return ""
	//return client.GenerateList("InventoryAdjustment", "InventoryAdjustment", "/Inventory/Adjustment")
}

// GenerateRetrieveInventoryAdjustment generates myob client code to retrieve specific feature
//
// API: /Inventory/Adjustment
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/inventory/adjustment/
func GenerateRetrieveInventoryAdjustment() string {
	return ""
	//return client.GenerateRetrieve("InventoryAdjustment", "/Inventory/Adjustment")
}

// GenerateInventoryAdjustmentModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/inventory/adjustment/
func GenerateInventoryAdjustmentModel() string {
	return client.GenerateModel("InventoryAdjustment", "https://developer.myob.com/api/myob-business-api/v2/inventory/adjustment/")
}
