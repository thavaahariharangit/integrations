package item

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListInventoryBillOfMaterials generates myob client code to fetch all features
//
// API: /Inventory/Item/?$expand=BillOfMaterials
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/inventory/item/bill-materials/
func GenerateListInventoryBillOfMaterials() string {
	return ""
	//return client.GenerateList("InventoryBillOfMaterials", "InventoryBillOfMaterials", "/Inventory/Item/?$expand=BillOfMaterials")
}

// GenerateRetrieveInventoryBillOfMaterials generates myob client code to retrieve specific feature
//
// API: /Inventory/Item/?$expand=BillOfMaterials
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/inventory/item/bill-materials/
func GenerateRetrieveInventoryBillOfMaterials() string {
	return ""
	//return client.GenerateRetrieve("InventoryBillOfMaterials", "/Inventory/Item/?$expand=BillOfMaterials")
}

// GenerateInventoryBillOfMaterialsModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/inventory/item/bill-materials/
func GenerateInventoryBillOfMaterialsModel() string {
	//return ""
	return client.GenerateModel("InventoryBillOfMaterials", "https://developer.myob.com/api/myob-business-api/v2/inventory/item/bill-materials/")
}
