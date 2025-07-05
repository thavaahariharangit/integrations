package main

import (
	"github.com/bitsnap/bitsnap-accounting-myob-codegen/codegen"

	"github.com/bitsnap/bitsnap-accounting-myob-codegen/myob"
	"github.com/bitsnap/myob-api-client/codegen/banking"
	"github.com/bitsnap/myob-api-client/codegen/company"
	"github.com/bitsnap/myob-api-client/codegen/contact"
	"github.com/bitsnap/myob-api-client/codegen/contact/employee"
	"github.com/bitsnap/myob-api-client/codegen/inventory"
	"github.com/bitsnap/myob-api-client/codegen/ledger"
	"github.com/bitsnap/myob-api-client/codegen/ledger/account"
	"github.com/bitsnap/myob-api-client/codegen/ledger/job"
	"github.com/bitsnap/myob-api-client/codegen/payroll"
	"github.com/bitsnap/myob-api-client/codegen/payroll/payroll_category"
	"github.com/bitsnap/myob-api-client/codegen/purchase"
	"github.com/bitsnap/myob-api-client/codegen/purchase/bill"
	"github.com/bitsnap/myob-api-client/codegen/purchase/order"
	"github.com/bitsnap/myob-api-client/codegen/sale"
	saleInvoice "github.com/bitsnap/myob-api-client/codegen/sale/invoice"
	saleOrder "github.com/bitsnap/myob-api-client/codegen/sale/order"
	saleQuote "github.com/bitsnap/myob-api-client/codegen/sale/quote"
	"github.com/bitsnap/myob-api-client/codegen/time_billing"
	"strings"

	goutil "github.com/bitsnap/bitsnap/packages/go-util"
)

type templateFunc struct {
	Model    func() string
	List     func() string
	Retrieve func() string
}

type templateFuncs map[string]templateFunc

func (t templateFuncs) toMap() map[string][]func() string {
	m := make(map[string][]func() string)
	for n, v := range t {
		m[n] = []func() string{
			v.Model,
			v.List,
			v.Retrieve,
		}
	}

	return m
}

var content = templateFuncs{
	"account_right.go": {
		myob.GenerateCompanyFilesModel,
		nil,
		myob.GenerateRetrieveCompanyFiles,
	},
	"account_right_info.go": {
		myob.GenerateInfoModel,
		nil,
		myob.GenerateRetrieveInfo,
	},
	"company.go": {
		myob.GenerateCompanyModel,
		nil,
		myob.GenerateRetrieveCompany,
	},
	"contact.go": {
		myob.GenerateContactModel,
		nil,
		myob.GenerateRetrieveContact,
	},
	"contact_photo.go": {
		nil,
		nil,
		myob.GenerateRetrieveContactPhoto,
	},
	"current_user.go": {
		myob.GenerateCurrentUserModel,
		nil,
		myob.GenerateRetrieveCurrentUser,
	},
	"banking_bank_account.go": {
		banking.GenerateBankAccountModel,
		banking.GenerateListBankAccount,
		banking.GenerateRetrieveBankAccount,
	},
	"banking_receive_money_txn.go": {
		banking.GenerateReceiveMoneyTransactionModel,
		banking.GenerateListMoneyTransaction,
		banking.GenerateReceiveMoneyTransaction,
	},
	"banking_spend_money_txn.go": {
		banking.GenerateSpendMoneyTransactionModel,
		banking.GenerateListSpendMoneyTransaction,
		banking.GenerateRetrieveSpendMoneyTransaction,
	},
	"banking_spend_money_txn_attachment.go": {
		nil,
		banking.GenerateListSpendMoneyTransactionAttachments,
		nil,
	},
	"banking_statement.go": {
		banking.GenerateStatementModel,
		banking.GenerateListStatement,
		banking.GenerateRetrieveStatement,
	},
	"banking_transfer_money_txn.go": {
		banking.GenerateTransferMoneyTransactionModel,
		banking.GenerateRetrieveTransferMoneyTransaction,
		banking.GenerateReceiveMoneyTransaction,
	},
	"company_custom_list.go": {
		company.GenerateCompanyCustomListModel,
		company.GenerateListCompanyCustomList,
		company.GenerateRetrieveCompanyCustomList,
	},
	"company_preferences.go": {
		company.GenerateCompanyPreferencesModel,
		company.GenerateListCompanyPreferences,
		company.GenerateRetrieveCompanyPreferences,
	},
	"contact_customer.go": {
		contact.GenerateCustomerContactModel,
		contact.GenerateListCustomerContact,
		contact.GenerateRetrieveCustomerContact,
	},
	"contact_employee.go": {
		contact.GenerateEmployeeContactModel,
		contact.GenerateListEmployeeContact,
		contact.GenerateRetrieveEmployeeContact,
	},
	"contact_employee_payment_details.go": {
		employee.GenerateEmployeePaymentDetailsModel,
		employee.GenerateListEmployeePaymentDetails,
		employee.GenerateRetrieveEmployeePaymentDetails,
	},
	"contact_employee_payroll_details.go": {
		employee.GenerateEmployeePayrollDetailsModel,
		employee.GenerateListEmployeePayrollDetails,
		employee.GenerateRetrieveEmployeePayrollDetails,
	},
	"contact_employee_standard_salary.go": {
		employee.GenerateEmployeeStandardSalaryModel,
		employee.GenerateListEmployeeStandardSalary,
		employee.GenerateRetrieveEmployeeStandardSalary,
	},
	"contact_personal.go": {
		contact.GeneratePersonalContactModel,
		contact.GenerateListPersonalContact,
		contact.GenerateRetrievePersonalContact,
	},
	"contact_supplier.go": {
		contact.GenerateSupplierContactModel,
		contact.GenerateListSupplierContact,
		contact.GenerateRetrieveSupplierContact,
	},
	"inventory_adjustment.go": {
		inventory.GenerateInventoryAdjustmentModel,
		inventory.GenerateListInventoryAdjustment,
		inventory.GenerateRetrieveInventoryAdjustment,
	},
	"inventory_build.go": {
		inventory.GenerateInventoryBuildModel,
		inventory.GenerateListInventoryBuild,
		inventory.GenerateRetrieveInventoryBuild,
	},
	"inventory_item.go": {
		inventory.GenerateInventoryItemModel,
		inventory.GenerateListInventoryItem,
		inventory.GenerateRetrieveInventoryItem,
	},
	"inventory_item_price_matrix.go": {
		inventory.GenerateItemPriceMatrixModel,
		inventory.GenerateListItemPriceMatrix,
		inventory.GenerateRetrieveItemPriceMatrix,
	},
	"inventory_location.go": {
		inventory.GenerateInventoryAdjustmentModel,
		inventory.GenerateListInventoryAdjustment,
		inventory.GenerateRetrieveInventoryAdjustment,
	},
	"ledger_account.go": {
		ledger.GenerateLedgerAccountModel,
		ledger.GenerateListLedgerAccount,
		ledger.GenerateRetrieveLedgerAccount,
	},
	"ledger_account_budget.go": {
		account.GenerateLedgerAccountBudgetModel,
		account.GenerateListLedgerAccountBudget,
		account.GenerateRetrieveLedgerAccountBudget,
	},
	"ledger_account_register.go": {
		account.GenerateLedgerAccountRegisterModel,
		account.GenerateListLedgerAccountRegister,
		account.GenerateRetrieveLedgerAccountRegister,
	},
	"ledger_accounting_properties.go": {
		account.GenerateLedgerAccountingPropertiesModel,
		account.GenerateListLedgerAccountingProperties,
		account.GenerateRetrieveLedgerAccountingProperties,
	},
	"ledger_category.go": {
		ledger.GenerateLedgerCategoryModel,
		ledger.GenerateListLedgerCategory,
		ledger.GenerateRetrieveLedgerCategory,
	},
	"ledger_category_register.go": {
		ledger.GenerateLedgerCategoryRegisterModel,
		ledger.GenerateListLedgerCategoryRegister,
		ledger.GenerateRetrieveLedgerCategoryRegister,
	},
	"ledger_currency.go": {
		ledger.GenerateLedgerCategoryModel,
		ledger.GenerateListLedgerCurrency,
		ledger.GenerateRetrieveLedgerCurrency,
	},
	"ledger_general_journal.go": {
		ledger.GenerateLedgerGeneralJournalModel,
		ledger.GenerateListLedgerGeneralJournal,
		ledger.GenerateRetrieveLedgerGeneralJournal,
	},
	"ledger_job.go": {
		ledger.GenerateLedgerJobModel,
		ledger.GenerateListLedgerJob,
		ledger.GenerateRetrieveLedgerJob,
	},
	"ledger_job_budget.go": {
		job.GenerateLedgerJobBudgetModel,
		job.GenerateListLedgerJobBudget,
		job.GenerateRetrieveLedgerJobBudget,
	},
	"ledger_job_register.go": {
		job.GenerateLedgerJobRegisterModel,
		job.GenerateListLedgerJobRegister,
		job.GenerateRetrieveLedgerJobRegister,
	},
	"ledger_journal_transaction.go": {
		ledger.GenerateLedgerJournalTransactionModel,
		ledger.GenerateListLedgerJournalTransaction,
		ledger.GenerateRetrieveLedgerJournalTransaction,
	},
	"ledger_linked_account.go": {
		ledger.GenerateLedgerLinkedAccountModel,
		ledger.GenerateListLedgerLinkedAccount,
		ledger.GenerateRetrieveLedgerLinkedAccount,
	},
	"ledger_profit_loss.go": {
		ledger.GenerateLedgerProfitLossDistributionModel,
		ledger.GenerateListLedgerProfitLossDistribution,
		ledger.GenerateRetrieveLedgerProfitLossDistribution,
	},
	"ledger_tax_code.go": {
		ledger.GenerateLedgerTaxCodeModel,
		ledger.GenerateListLedgerTaxCode,
		ledger.GenerateRetrieveLedgerTaxCode,
	},
	"payroll_employment_classification.go": {
		payroll.GenerateEmploymentClassificationModel,
		payroll.GenerateListEmploymentClassification,
		payroll.GenerateRetrieveEmploymentClassification,
	},
	"payroll_category.go": {
		payroll.GeneratePayrollCategoryModel,
		payroll.GenerateListPayrollCategory,
		payroll.GenerateRetrievePayrollCategory,
	},
	"payroll_category_deduction.go": {
		nil,
		nil,
		payroll_category.GenerateRetrievePayrollCategoryDeduction,
	},
	"payroll_category_entitlement.go": {
		nil,
		nil,
		payroll_category.GenerateRetrievePayrollCategoryEntitlement,
	},
	"payroll_category_expense.go": {
		nil,
		nil,
		payroll_category.GenerateRetrievePayrollCategoryExpense,
	},
	"payroll_category_superannuation.go": {
		nil,
		nil,
		payroll_category.GenerateRetrievePayrollCategorySuperannuation,
	},
	"payroll_category_tax.go": {
		nil,
		nil,
		payroll_category.GenerateRetrievePayrollCategoryTax,
	},
	"payroll_category_tax_table.go": {
		nil,
		nil,
		payroll_category.GenerateRetrievePayrollCategoryTaxTable,
	},
	"payroll_category_wage.go": {
		nil,
		nil,
		payroll_category.GenerateRetrievePayrollCategoryWage,
	},
	"payroll_superannuation_fund.go": {
		payroll.GenerateSuperannuationFundModel,
		payroll.GenerateListSuperannuationFund,
		payroll.GenerateRetrieveSuperannuationFund,
	},
	"payroll_timesheet.go": {
		payroll.GenerateTimesheetModel,
		payroll.GenerateListTimesheet,
		payroll.GenerateRetrieveTimesheet,
	},
	"purchase_bill.go": {
		purchase.GeneratePurchaseBillModel,
		purchase.GenerateListPurchaseBill,
		purchase.GenerateRetrievePurchaseBill,
	},
	"purchase_bill_item.go": {
		bill.GenerateBillItemModel,
		bill.GenerateListBillItem,
		bill.GenerateRetrieveBillItem,
	},
	"purchase_bill_item_attachment.go": {
		bill.GenerateBillItemAttachmentModel,
		nil,
		bill.GenerateRetrieveBillItemAttachment,
	},
	"purchase_bill_misc.go": {
		bill.GenerateBillMiscellaneousModel,
		bill.GenerateListBillMiscellaneous,
		bill.GenerateRetrieveBillMiscellaneous,
	},
	"purchase_bill_misc_attachment.go": {
		bill.GenerateBillMiscellaneousAttachmentModel,
		nil,
		bill.GenerateRetrieveBillMiscellaneousAttachment,
	},
	"purchase_bill_professional.go": {
		bill.GenerateBillProfessionalModel,
		bill.GenerateListBillProfessional,
		bill.GenerateRetrieveBillProfessional,
	},
	"purchase_bill_professional_attachment.go": {
		nil,
		nil,
		bill.GenerateRetrieveBillProfessionalAttachment,
	},
	"purchase_bill_service.go": {
		bill.GenerateBillServiceModel,
		bill.GenerateListBillService,
		bill.GenerateRetrieveBillService,
	},
	"purchase_bill_service_attachment.go": {
		nil,
		nil,
		bill.GenerateRetrieveBillServiceAttachment,
	},
	"purchase_debit_refund.go": {
		purchase.GenerateDebitRefundModel,
		purchase.GenerateListDebitRefund,
		purchase.GenerateRetrieveDebitRefund,
	},
	"purchase_debit_settlement.go": {
		purchase.GenerateDebitSettlementModel,
		purchase.GenerateListDebitSettlement,
		purchase.GenerateRetrieveDebitSettlement,
	},
	"purchase_order.go": {
		purchase.GenerateOrderModel,
		purchase.GenerateListOrder,
		purchase.GenerateRetrieveOrder,
	},
	"purchase_order_item.go": {
		order.GenerateOrderItemModel,
		order.GenerateListOrderItem,
		order.GenerateRetrieveOrderItem,
	},
	"purchase_order_misc.go": {
		order.GenerateOrderMiscellaneousModel,
		order.GenerateListOrderMiscellaneous,
		order.GenerateRetrieveOrderMiscellaneous,
	},
	"purchase_order_professional.go": {
		order.GenerateOrderProfessionalModel,
		order.GenerateListOrderProfessional,
		order.GenerateRetrieveOrderProfessional,
	},
	"purchase_order_service.go": {
		order.GenerateOrderServiceModel,
		order.GenerateListOrderService,
		order.GenerateRetrieveOrderService,
	},
	"purchase_supplier_payment.go": {
		purchase.GenerateSupplierPaymentModel,
		purchase.GenerateListSupplierPayment,
		purchase.GenerateRetrieveSupplierPayment,
	},
	"sale_credit_refund.go": {
		purchase.GenerateSupplierPaymentModel,
		purchase.GenerateListSupplierPayment,
		purchase.GenerateRetrieveSupplierPayment,
	},
	"sale_credit_settlement.go": {
		sale.GenerateSaleCreditSettlementModel,
		sale.GenerateListSaleCreditSettlement,
		nil,
	},
	"sale_customer_payment.go": {
		sale.GenerateSaleCustomerPaymentModel,
		sale.GenerateListSaleCustomerPayment,
		nil,
	},
	"sale_invoice.go": {
		sale.GenerateSaleInvoiceModel,
		sale.GenerateListSaleInvoice,
		nil,
	},
	"sale_invoice_item.go": {
		saleInvoice.GenerateSaleInvoiceItemModel,
		saleInvoice.GenerateListSaleInvoiceItem,
		nil,
	},
	"sale_invoice_misc.go": {
		saleInvoice.GenerateSaleInvoiceMiscellaneousModel,
		saleInvoice.GenerateListSaleInvoiceMiscellaneous,
		nil,
	},
	"sale_invoice_professional.go": {
		saleInvoice.GenerateInvoiceSaleProfessionalModel,
		saleInvoice.GenerateListSaleInvoiceProfessional,
		nil,
	},
	"sale_invoice_service.go": {
		saleInvoice.GenerateSaleInvoiceServiceModel,
		saleInvoice.GenerateListSaleInvoiceService,
		nil,
	},
	"sale_invoice_time_billing.go": {
		saleInvoice.GenerateInvoiceTimeBillingModel,
		saleInvoice.GenerateListInvoiceTimeBilling,
		nil,
	},
	"sale_order.go": {
		sale.GenerateSaleOrderModel,
		sale.GenerateListSaleOrder,
		nil,
	},
	"sale_order_item.go": {
		saleOrder.GenerateSaleOrderItemModel,
		saleOrder.GenerateListSaleOrderItem,
		nil,
	},
	"sale_order_misc.go": {
		saleOrder.GenerateOrderMiscellaneousModel,
		saleOrder.GenerateListOrderMiscellaneous,
		nil,
	},
	"sale_order_professional.go": {
		saleOrder.GenerateOrderProfessionalModel,
		saleOrder.GenerateListOrderProfessional,
		nil,
	},
	"sale_order_service.go": {
		saleOrder.GenerateSaleOrderServiceModel,
		saleOrder.GenerateListSaleOrderService,
		nil,
	},
	"sale_order_time_billing.go": {
		saleOrder.GenerateOrderTimeBillingModel,
		saleOrder.GenerateListOrderTimeBilling,
		nil,
	},
	"sale_quote.go": {
		sale.GenerateSaleQuoteModel,
		sale.GenerateListSaleQuote,
		nil,
	},
	"sale_quote_item.go": {
		saleQuote.GenerateSaleQuoteItemModel,
		saleQuote.GenerateListSaleQuoteItem,
		nil,
	},
	"sale_quote_misc.go": {
		saleQuote.GenerateListSaleQuoteMiscellaneous,
		saleQuote.GenerateListSaleQuoteMiscellaneous,
		nil,
	},
	"sale_quote_professional.go": {
		saleQuote.GenerateSaleQuoteProfessionalModel,
		saleQuote.GenerateListSaleQuoteProfessional,
		nil,
	},
	"sale_quote_service.go": {
		saleQuote.GenerateSaleQuoteServiceModel,
		saleQuote.GenerateListSaleQuoteService,
		nil,
	},
	"sale_quote_time_billing.go": {
		saleQuote.GenerateSaleQuoteTimeBillingModel,
		saleQuote.GenerateListSaleQuoteTimeBilling,
		nil,
	},
	"time_billing_activity.go": {
		time_billing.GenerateTimeBillingActivityModel,
		time_billing.GenerateListTimeBillingActivity,
		nil,
	},
	"time_billing_activity_slip.go": {
		time_billing.GenerateTimeBillingActivitySlipModel,
		time_billing.GenerateListTimeBillingActivitySlip,
		nil,
	},
}

func header(content string) string {
	enums := "\t\"github.com/bitsnap/bitsnap/packages/data-pipeline-general-accounting/pkg/integrations/freshbooks/enums\"\n"
	models := "\t\"github.com/bitsnap/bitsnap/packages/data-pipeline-general-accounting/pkg/integrations/freshbooks/models\"\n"
	decimal := "\t\"github.com/shopspring/decimal\"\n"
	time := "\t\"time\"\n"

	if !strings.Contains(content, "models.") {
		models = ""
	}

	if !strings.Contains(content, "enums.") {
		enums = ""
	}

	if !strings.Contains(content, "time.") {
		time = ""
	}

	if !strings.Contains(content, "decimal.") {
		decimal = ""
	}

	return `package myob

// THIS IS GENERATED CODE. DO NOT EDIT.

import (
` + enums + models + decimal + time + `
	"github.com/goccy/go-json"
	"github.com/gofiber/fiber/v2"
	"net/url"
)

`
}

func GenerateInto(targetDir string) {
	for _, f := range content {
		if f.Model != nil {
			f.Model()
		}
	}

	// We're removing duplicated parent model attributes in here
	//scraper.CleanupAttributes()

	goutil.GenerateInto(targetDir, header, content.toMap())
}

func main() {
	dir := TargetCodegenDir()
	syncLogger := SetGlobalLogger()
	defer syncLogger()

	Logger().Infow("starting bitsnap myob codegen",
		"dir", dir,
	)

	codegen.GenerateInto(dir)
}
