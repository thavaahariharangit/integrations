use crate::models::*;

pub fn get() -> Vec<Model> {
    vec![
        Model {
            name: "Amount".to_string(),
            fields: vec![
                Field {
                    field_name: "amount".to_string(),
                    field_type: "BigDecimal".to_string(),
                    is_list: false,
                    is_optional: false,
                    submodel: None,
                },
                Field {
                    field_name: "code".to_string(),
                    field_type: "String".to_string(),
                    is_list: false,
                    is_optional: false,
                    submodel: None,
                },
            ]
            .iter()
            .map(|f| f.clone())
            .collect(),
        },
        Model {
            name: "OtherIncomeTax".to_string(),
            fields: vec![
                Field {
                    field_name: "amount".to_string(),
                    field_type: "BigDecimal".to_string(),
                    is_list: false,
                    is_optional: false,
                    submodel: None,
                },
                Field {
                    field_name: "name".to_string(),
                    field_type: "String".to_string(),
                    is_list: false,
                    is_optional: false,
                    submodel: None,
                },
            ]
            .iter()
            .map(|f| f.clone())
            .collect(),
        },
    ]
}

pub fn replaced_with_common(name: &str) -> Option<Model> {
    let common = get();
    let ammount = common[0].clone();
    let other_tax = common[1].clone();

    vec![
        ("EstimateDiscountTotal", ammount.clone()),
        ("InvoiceProfileDiscountTotal", ammount.clone()),
        ("InvoiceDepositAmount", ammount.clone()),
        ("OtherIncomeAmount", ammount.clone()),
        ("PaymentAmount", ammount.clone()),
        ("OtherIncomeTaxes", other_tax.clone()),
    ]
    .iter()
    .find(|(n, _)| *n == name)
    .map(|(_, m)| m.clone())
}

pub fn replace_with_empty(model_name: &str, field_name: &str, field_type: &str) -> Option<Model> {
    vec![
        ("Estimate", "lines", "String", "EstimateLine"),
        ("Project", "members", "ProjectMembers", "ProjectMember"),
        ("Project", "services", "ProjectServices", "ProjectService"),
    ]
    .iter()
    .find(|(n, f, t, _)| *n == model_name && *f == field_name && *t == field_type)
    .map(|(_, _, _, name)| Model {
        name: name.to_string(),
        fields: HashSet::new(),
    })
}

pub fn replace_common_model<'a>(field: Field, model_name: &str) -> Field {
    let mut field = field.clone();

    // eprintln!("x {} {} {}", model_name, field.field_name, field.field_type);

    let empty = replace_with_empty(model_name, &field.field_name, &field.field_type);
    if let Some(to_replace) = empty {
        eprintln!(
            "replacing {} {} with empty model {}",
            model_name, field.field_name, to_replace.name
        );
        field.field_type = to_replace.name.clone();
        field.submodel = Some(to_replace.clone());

        return field;
    }

    if let Some(submodel) = field.submodel.clone() {
        let replaced = replaced_with_common(&submodel.name);
        let common = get()
            .iter()
            .find(|r| r.fields == submodel.fields)
            .map(|r| r.clone());

        if let Some(to_replace) = common.or(replaced) {
            eprintln!("replacing {} with model {}", submodel.name, to_replace.name);
            field.field_type = to_replace.name.clone();
            field.submodel = Some(to_replace.clone());
        }
    }

    field
}
