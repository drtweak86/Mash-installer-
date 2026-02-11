use anyhow::Result;
use installer_core::catalog::Catalog;
use serde_json;

pub fn catalog_to_text(catalog: &Catalog) -> String {
    let mut output = String::new();
    for category in &catalog.categories {
        output.push_str(&format!("{}\n", category.name));
        for option in &category.options {
            let default_marker = if option.default { " (default)" } else { "" };
            output.push_str(&format!(
                "  - {}{}: {}\n",
                option.name, default_marker, option.description
            ));
        }
        output.push('\n');
    }
    output
}

pub fn catalog_to_json(catalog: &Catalog) -> serde_json::Result<String> {
    serde_json::to_string_pretty(catalog)
}

pub fn print_catalog(catalog: &Catalog, json_output: bool) -> Result<()> {
    if json_output {
        println!("{}", catalog_to_json(catalog)?);
    } else {
        print!("{}", catalog_to_text(catalog));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::catalog_to_text;
    use super::catalog_to_json;
    use installer_core::catalog::Catalog;

    #[test]
    fn catalog_output_includes_categories() {
        let catalog = Catalog::curated();
        assert!(!catalog.categories.is_empty());
        let text = catalog_to_text(&catalog);
        assert!(text.contains(catalog.categories[0].name));
    }

    #[test]
    fn catalog_json_is_valid() {
        let catalog = Catalog::curated();
        let json = catalog_to_json(&catalog).expect("json");
        assert!(serde_json::from_str::<serde_json::Value>(&json).is_ok());
    }
}
