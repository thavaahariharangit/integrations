package client

import (
	"fmt"
	"net/url"
	"os"
	"strconv"
	"time"

	"github.com/goccy/go-json"
	"github.com/gofiber/fiber/v2"
)

//go:generate go run ../../codegen/main.go ../..

var tokenToUse = ""

func UseToken(token string) {
	if tokenToUse != "" {
		panic("CHARGEBEE_API_TOKEN had been set already")
	}

	tokenToUse = token
}

func Token() string {
	if tokenToUse != "" {
		return tokenToUse
	}

	if tok, ok := os.LookupEnv("CHARGEBEE_API_TOKEN"); ok {
		return tok
	}

	panic("CHARGEBEE_API_TOKEN environment variable is missing")
}

var siteToUse = ""

func UseSite(site string) {
	if siteToUse != "" {
		panic("CHARGEBEE_SITE had been set already")
	}

	siteToUse = site
}

func Site() string {
	if siteToUse != "" {
		return siteToUse
	}

	if tok, ok := os.LookupEnv("CHARGEBEE_SITE"); ok {
		return tok
	}

	panic("CHARGEBEE_SITE environment variable is missing")
}

func GetQuery(client *fiber.Client, parsedUrl *url.URL, offset string, sortBy *SortBy) ([]byte, error) {
	q := parsedUrl.Query()
	if offset != "" {
		q.Add("offset", offset)
	}

	if sortBy != nil {
		if sortBy.Name != "" {
			if sortBy.Asc {
				q.Add("sort_by[asc]", sortBy.Name)
			} else {
				q.Add("sort_by[desc]", sortBy.Name)
			}
		}

		if sortBy.AfterDate != nil {
			if sortBy.UpdatedAfter {
				if sortBy.Asc {
					q.Add("sort_by[asc]", "updated_at")
				} else {
					q.Add("sort_by[desc]", "updated_at")
				}

				q.Add("updated_at[after]", strconv.Itoa(int(sortBy.AfterDate.Unix())))
				q.Add("updated_at[before]", strconv.Itoa(int(time.Now().Unix())))
			} else {
				if sortBy.Asc {
					q.Add("sort_by[asc]", "created_at")
				} else {
					q.Add("sort_by[desc]", "created_at")
				}

				q.Add("created_at[after]", strconv.Itoa(int(sortBy.AfterDate.Unix())))
				q.Add("created_at[before]", strconv.Itoa(int(time.Now().Unix())))
			}
		}
	}

	parsedUrl.RawQuery = q.Encode()

	statusCode, content, errs := client.Get(parsedUrl.String()).BasicAuth(Token(), "").MaxRedirectsCount(5).Timeout(time.Second * 10).Bytes()
	if len(errs) > 0 || statusCode != 200 {
		return nil, fmt.Errorf("%v \nget failed, status code: %d, body: %s", parsedUrl.String(), statusCode, content)
	}

	return content, nil
}

func ResultWithOffset[R any](result []R, offset, nextOffset string) ([]R, string, error) {
	nextOffsetVal := make([]string, 0, 10)

	// NOTE: some offsets are represented as Numbers while others as stringified JSON Arrays of string offsets :\
	err := json.Unmarshal([]byte(nextOffset), &nextOffsetVal)
	if err != nil {
		nextOffsetValNum := 1
		err := json.Unmarshal([]byte(nextOffset), &nextOffsetValNum)
		if err != nil {
			return nil, "", err
		}

		nextOffsetVal = append(nextOffsetVal, strconv.Itoa(nextOffsetValNum))
	}

	if offset != "" {
		for i := range nextOffsetVal {
			if nextOffsetVal[i] == offset {
				if i+1 != len(nextOffsetVal) {
					return result, nextOffsetVal[i+1], nil
				}

				break
			}
		}

		return result, "", nil
	}

	return result, nextOffsetVal[0], nil
}
