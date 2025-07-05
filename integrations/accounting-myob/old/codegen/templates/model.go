package templates

import (
	"github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/scraper"
)

func GenerateModel(name, page string) string {
	model, err := scraper.ScrapeModel(name, "", page)
	if err != nil {
		panic(err)
	}

	return model.String()
}

func GenerateModelWithParent(name, parentName, page string) string {
	model, err := scraper.ScrapeModel(name, parentName, page)
	if err != nil {
		panic(err)
	}

	return model.String()
}
