package client

import (
	"time"
)

type SortBy struct {
	Asc  bool
	Name string

	AfterDate    *time.Time
	UpdatedAfter bool
}

var defaultSortBy = &SortBy{
	Name: "updated_at",
}

func DefaultSortBy() *SortBy {
	return defaultSortBy
}

func ListAll[T any](site string, sortBy *SortBy, lister func(site string, offset string, sortBy *SortBy) ([]T, string, error)) ([]T, error) {
	results := make([]T, 0, 10)

	result, tag, err := lister(site, "", sortBy)
	results = append(results, result...)
	if err != nil {
		return results, err
	}

	for ; err == nil && len(result) > 0 && tag != ""; result, tag, err = lister(site, tag, sortBy) {
		results = append(results, result...)
	}

	if err != nil {
		return results, err
	}

	return results, err
}

func ListAllOfId[T any](site, id string, sortBy *SortBy, lister func(site string, id string, offset string, sortBy *SortBy) ([]T, string, error)) ([]T, error) {
	results := make([]T, 0, 10)

	result, tag, err := lister(site, id, "", sortBy)

	results = append(results, result...)
	if err != nil {
		return results, err
	}

	for ; err == nil && len(result) > 0 && tag != ""; result, tag, err = lister(site, id, tag, sortBy) {
		results = append(results, result...)
	}

	if err != nil {
		return results, err
	}

	return results, err
}
