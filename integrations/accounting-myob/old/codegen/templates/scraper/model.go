package scraper

import (
	"regexp"
	"strconv"
	"strings"

	"github.com/PuerkitoBio/goquery"
	. "github.com/bitsnap/bitsnap/packages/go-util"
	"github.com/gocolly/colly/v2"
)

type Model struct {
	Name       string
	ParentName string
	Attributes []Attribute
}

func (m *Model) String() string {
	attrs := make([]string, 0, len(m.Attributes))
	for _, attr := range m.Attributes {
		reqSuffix := "`"
		if attr.Required {
			reqSuffix = " validation:\"required\"`"
		}
		attrs = append(attrs, "\t"+attr.Name+"\t"+attr.Type+" `json:\""+attr.Name+"\""+reqSuffix+"\t // "+attr.Description)
	}

	return "type " + m.Name + " struct {\n" + strings.Join(attrs, "\n") + "\n}"
}

var knownModels = make(map[string]*Model)

var lengthRegex = regexp.MustCompile(`.*\(\s*(\d+)\s*\)\s*$`)

func ScrapeModel(modelName, parentName, page string) (*Model, error) {
	collector := colly.NewCollector(
		colly.AllowedDomains("developer.myob.com", "myob.com"),
		colly.CacheDir("./.cache"),
	)

	attributes := make([]Attribute, 0, 10)

	prefix := modelName
	if parentName != "" {
		prefix = parentName + Capitalize(modelName)
	}

	if knownModels[Capitalize(modelName)] != nil {
		return knownModels[Capitalize(modelName)], nil
	}

	collector.OnHTML("div.documentation", func(docs *colly.HTMLElement) {
		docs.DOM.Find("h4").FilterFunction(func(_ int, header *goquery.Selection) bool {
			return strings.TrimSpace(strings.ToLower(header.Text())) == "attribute details"
		}).First().NextAllFiltered("ul.apiDocs").First().Each(func(_ int, table *goquery.Selection) {
			table.Find("li.attribute").Each(func(_ int, attr *goquery.Selection) {
				if cls, ok := attr.Parent().Parent().Attr("class"); ok && !strings.Contains(cls, "subgroupDetails") {
					t := strings.TrimSpace(attr.Find("span.type").First().Text())
					name := strings.TrimSpace(strings.Replace(attr.Text(), t, "", 1))
					t = strings.ToLower(t)

					desc := strings.TrimSpace(attr.NextAllFiltered("li.description").First().Text())
					length := 0
					if lengthRegex.MatchString(t) {
						l := lengthRegex.FindStringSubmatch(t)
						length, _ = strconv.Atoi(l[len(l)-1])

						if length > 0 {
							t = strings.TrimSpace(strings.Replace(t, "("+strconv.Itoa(length)+")", "", -1))
						}
					}

					if t == "" {
						t = "models." + Capitalize(prefix)
					} else {
						t = formatType(t)
					}

					attributes = append(attributes, *NewAttribute(name, t, desc, attr.HasClass("required"), length))
				}
			})
		})
	})

	collector.OnError(func(response *colly.Response, err error) {
		Logger().Errorw(response.Request.URL.String(), err)
	})

	err := collector.Visit(page)
	if err != nil {
		return nil, err
	}

	m := &Model{
		Name:       Capitalize(modelName),
		ParentName: Capitalize(parentName),
		Attributes: attributes,
	}

	if _, ok := knownModels[m.Name]; ok {
		Logger().Infow("already scraped model", "model", m)
	} else {
		knownModels[m.Name] = m
	}

	return m, nil
}
