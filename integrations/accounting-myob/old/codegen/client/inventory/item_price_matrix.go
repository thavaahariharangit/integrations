package inventory

import "github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/client"

// GenerateListItemPriceMatrix generates myob client code to fetch all features
//
// API: /Inventory/ItemPriceMatrix
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/inventory/item_price_matrix/
func GenerateListItemPriceMatrix() string {
	return ""
	//return client.GenerateList("ItemPriceMatrix", "ItemPriceMatrix", "/Inventory/ItemPriceMatrix")
}

// GenerateRetrieveItemPriceMatrix generates myob client code to retrieve specific feature
//
// API: /Inventory/ItemPriceMatrix
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/inventory/item_price_matrix/
func GenerateRetrieveItemPriceMatrix() string {
	return ""
	//return client.GenerateRetrieve("ItemPriceMatrix", "/Inventory/ItemPriceMatrix")
}

// GenerateItemPriceMatrixModel generates myob feature domain model
//
// Documentation: https://developer.myob.com/api/myob-business-api/v2/inventory/item_price_matrix/
func GenerateItemPriceMatrixModel() string {
	//return ""
	return client.GenerateModel("ItemPriceMatrix", "https://developer.myob.com/api/myob-business-api/v2/inventory/item_price_matrix/")
}
