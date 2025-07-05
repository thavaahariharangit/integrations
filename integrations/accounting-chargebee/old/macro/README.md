### ChargeBee API Client

ChargeBee API client crate.

### Testing

```bash
cargo test -p bitsnap_accounting_chargebee_codegen
CHARGEBEE_API_TOKEN=test_xxx CHARGEBEE_SITE=xxx-test cargo test -p bitsnap_accounting_chargebee

# to update goldenfiles
GOLDIE_UPDATE=1 cargo test -p bitsnap_accounting_chargebee_codegen
GOLDIE_UPDATE=1 cargo test -p bitsnap_accounting_chargebee
```

### TODO
 - client endpoints

## License
