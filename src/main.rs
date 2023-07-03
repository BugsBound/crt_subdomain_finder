use anyhow::bail;
use serde_json::Value;
use std::collections::HashSet;

async fn check_crt(domain: &str) -> anyhow::Result<HashSet<String>> {
    let finding_domain = format!(".{}", domain);
    let client = reqwest::Client::new();
    let params = [
        ("q", format!("%{}", finding_domain)),
        ("output", "json".to_string()),
    ];

    let req = client.get("https://crt.sh/").query(&params).send().await?;

    if !req.status().is_success() {
        bail!("Fail to get domains!");
    }

    let res: Vec<Value> = req.json().await?;

    let mut domains = HashSet::new();

    for entry in res {
        if let Some(Value::String(name_value)) = entry.get("name_value") {
            if name_value.ends_with(&finding_domain) {
                for check in name_value.split('\n') {
                    if is_valid_name(check, &finding_domain) {
                        domains.insert(check.to_string());
                    }
                }
            }
        }
    }

    Ok(domains)
}

fn is_valid_name(name: &str, finding_domain: &str) -> bool {
    name.ends_with(finding_domain) && !name.starts_with('*')
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut args = std::env::args();
    args.next();

    let domain = match args.next() {
        Some(s) => s,
        None => bail!("Need domain!"),
    };

    let domains = check_crt(&domain).await?;

    if domains.is_empty() {
        println!("Not found!");
    } else {
        println!("Founded {} domains!\n---", domains.len());

        for domain in domains {
            println!("{}", domain);
        }
    }

    Ok(())
}
