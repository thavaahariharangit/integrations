package item_price_matrix

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListInventoryPriceLevelDetail generates myob client code to fetch all features
//
// API: /Inventory/PriceLevelDetail
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/inventory/item_price_matrix/price_level_detail/
func GenerateListInventoryPriceLevelDetail() string {
	return ""
	//return client.GenerateList("InventoryPriceLevelDetail", "InventoryPriceLevelDetail", "/Inventory/PriceLevelDetail")
}

// GenerateRetrieveInventoryPriceLevelDetail generates myob client code to retrieve specific feature
//
// API: /Inventory/PriceLevelDetail
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/inventory/item_price_matrix/price_level_detail/
func GenerateRetrieveInventoryPriceLevelDetail() string {
	return ""
	//return client.GenerateRetrieve("InventoryPriceLevelDetail", "/Inventory/PriceLevelDetail")
}

// GenerateInventoryPriceLevelDetailModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/inventory/item_price_matrix/price_level_detail/
func GenerateInventoryPriceLevelDetailModel() string {
	//return ""
	return client.GenerateModel("InventoryPriceLevelDetail", "https://developer.myob.com/api/myob-business-api/v2/inventory/item_price_matrix/price_level_detail/")
}
