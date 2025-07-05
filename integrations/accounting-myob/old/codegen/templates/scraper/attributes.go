package scraper

import (
	"fmt"
	"strconv"
	"strings"

	. "github.com/bitsnap/bitsnap/packages/go-util"
)

type Attribute struct {
	Name        string
	Length      int
	Type        string
	Description string
	Required    bool
}

func (a *Attribute) String() string {
	t := a.Type
	if a.Length > 0 {
		t = t + " (" + strconv.Itoa(a.Length) + ")"
	}

	if a.Required {
		return fmt.Sprintf("%v required %v", a.Name, t)
	}

	return fmt.Sprintf("%v %v %v", a.Name, t, a.Description)
}

var fieldsNamesSingular = map[string]string{
	"Addresses":         "Address",
	"Lines":             "Line",
	"Budgets":           "Budget",
	"Identifiers":       "Identifier",
	"BankAccounts":      "BankAccount",
	"LocationDetails":   "LocationDetail",
	"Invoices":          "Invoice",
	"SellingPrices":     "SellingPrice",
	"EmployerExpenses":  "EmployerExpense",
	"UserAccess":        "UserAccess",
	"PayrollCategories": "PayrollCategory",
	"Entitlements":      "Entitlement",
	"Deductions":        "Deduction",
	"Resources":         "Resource",
}

func NewAttribute(name, t, description string, required bool, length int) *Attribute {
	arr := false
	if strings.HasSuffix(name, "[]") {
		name = strings.TrimSuffix(name, "[]")
		name = strings.TrimSpace(name)
		arr = true
	}

	if arr {
		if n, ok := fieldsNamesSingular[name]; !ok {
			Logger().Warnln(n, name, "singular field name is not found")
		} else {
			name = n
		}
	}

	descComp := make([]string, 0, 5)
	for _, d := range strings.Split(description, "\n") {
		c := strings.TrimSpace(d)
		if c != "" {
			descComp = append(descComp, c)
		}
	}

	description = strings.Join(descComp, " ")
	description = strings.ReplaceAll(description, "\t", " ")

	return &Attribute{
		Name:        name,
		Length:      length,
		Type:        t,
		Description: description,
		Required:    required,
	}
}
