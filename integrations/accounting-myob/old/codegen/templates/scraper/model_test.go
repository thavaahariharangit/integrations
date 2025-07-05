package scraper_test

import (
	"github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen/scraper"
	. "github.com/onsi/ginkgo/v2"
	. "github.com/onsi/gomega"
	"github.com/tommy351/goldga"
)

var _ = Describe("Model attributes scraping", func() {
	When("traversing documentation pages", func() {
		It("should be able to scrape model from documentation page", func() {
			model, err := scraper.ScrapeModel("CustomerContract", "", "https://developer.myob.com/api/myob-business-api/v2/contact/customer/")

			Expect(err).To(Not(HaveOccurred()))
			Expect(model.String()).To(goldga.Match())
		})
	})
})
