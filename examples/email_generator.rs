//! Email Address Generator Example
//!
//! This example demonstrates how to use markov-rs to generate realistic-looking
//! email addresses by training on patterns of names and domains.
//!
//! Useful for:
//! - Generating test data
//! - Creating mock user accounts
//! - Populating development databases
//! - Privacy-preserving sample data
//!
//! Run with: cargo run --example email_generator

use markov_rs::Chain;

/// Generate email addresses using a pattern-based approach
#[allow(dead_code)]
struct EmailGenerator {
    name_chain: Chain,
    domain_chain: Chain,
    domains: Vec<String>,
}

impl EmailGenerator {
    /// Create a new email generator from lists of names and domains
    fn new(names: &[&str], domains: &[&str]) -> Self {
        // Build name chain (character-level for more variety)
        let name_corpus: Vec<Vec<String>> = names
            .iter()
            .map(|name| name.chars().map(|c| c.to_string()).collect::<Vec<_>>())
            .collect();

        let name_chain = Chain::new(&name_corpus, 2);

        // Build domain chain
        let domain_corpus: Vec<Vec<String>> = domains
            .iter()
            .map(|domain| domain.chars().map(|c| c.to_string()).collect::<Vec<_>>())
            .collect();

        let domain_chain = Chain::new(&domain_corpus, 2);

        EmailGenerator {
            name_chain,
            domain_chain,
            domains: domains.iter().map(|s| s.to_string()).collect(),
        }
    }

    /// Generate a random name part
    fn generate_name(&self, max_len: usize) -> String {
        let chars = self.name_chain.walk(None);
        let name: String = chars
            .into_iter()
            .take(max_len)
            .filter(|c| c.chars().all(|ch| ch.is_alphanumeric()))
            .collect();

        // Ensure we have at least some characters
        if name.is_empty() {
            "user".to_string()
        } else {
            name
        }
    }

    /// Generate a random domain
    fn generate_domain(&self) -> String {
        let chars = self.domain_chain.walk(None);
        let domain: String = chars.into_iter().collect();

        // Ensure we have at least some characters
        if domain.is_empty() {
            "example.com".to_string()
        } else {
            domain
        }
    }

    /// Generate a complete email address
    fn generate_email(&self, max_name_len: usize) -> String {
        let name = self.generate_name(max_name_len);
        let domain = self.generate_domain();
        format!("{}@{}", name, domain)
    }

    /// Generate multiple email addresses
    fn generate_emails(&self, count: usize, max_name_len: usize) -> Vec<String> {
        (0..count)
            .map(|_| self.generate_email(max_name_len))
            .collect()
    }

    /// Generate email with a specific pattern
    fn generate_pattern_email(&self, pattern: &str) -> String {
        let name = self.generate_name(12);

        // Replace placeholders in pattern
        pattern
            .replace("{name}", &name)
            .replace("{domain}", &self.generate_domain())
    }
}

/// Alternative approach: Use Text model with email patterns
fn generate_emails_with_text(emails: &[&str], count: usize) -> Vec<String> {
    use markov_rs::NewlineText;

    // Create model from existing email patterns (newline delimited)
    let corpus = emails.join("\n");
    let model = NewlineText::new(&corpus, 1, true, false, None).expect("Failed to create model");

    let mut generated = Vec::with_capacity(count);
    let mut attempts = 0;

    while generated.len() < count && attempts < count * 10 {
        attempts += 1;
        if let Some(email) =
            model.make_sentence(None, Some(20), None, None, Some(false), Some(20), Some(5))
        {
            // Clean up the generated email
            let email = email
                .chars()
                .filter(|c| c.is_alphanumeric() || *c == '@' || *c == '.' || *c == '_' || *c == '-')
                .collect::<String>();

            // Validate email format: must have @ and . after @
            if email.contains('@') {
                let parts: Vec<&str> = email.split('@').collect();
                if parts.len() == 2 && parts[1].contains('.') && !parts[1].starts_with('.') {
                    generated.push(email);
                }
            }
        }
    }

    generated
}

/// Simple email generator using pattern combination
fn generate_simple_emails(first_names: &[&str], domains: &[&str], count: usize) -> Vec<String> {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    let separators = ["", ".", "_", ""];
    let suffixes = ["", "123", "2024", ""];

    (0..count)
        .map(|_| {
            let name = first_names[rng.gen_range(0..first_names.len())];
            let domain = domains[rng.gen_range(0..domains.len())];
            let sep = separators[rng.gen_range(0..separators.len())];
            let suffix = suffixes[rng.gen_range(0..suffixes.len())];

            format!("{}{}{}@{}", name, sep, suffix, domain)
        })
        .collect()
}

fn main() {
    println!("=== Email Address Generator Example ===\n");

    // Sample data for training
    let first_names = vec![
        "john", "jane", "alice", "bob", "charlie", "diana", "edward", "fiona", "george", "helen",
        "ivan", "julia", "kevin", "laura", "michael", "nina", "oscar", "patricia", "quinn",
        "rachel", "steven", "tina", "ulrich", "victoria", "william", "xena", "yuri", "zoe",
        "david", "emma", "frank", "grace",
    ];

    let _last_names = vec![
        "smith",
        "johnson",
        "williams",
        "brown",
        "jones",
        "garcia",
        "miller",
        "davis",
        "rodriguez",
        "martinez",
        "hernandez",
        "lopez",
        "gonzalez",
        "wilson",
        "anderson",
        "thomas",
        "taylor",
        "moore",
        "jackson",
        "martin",
        "lee",
        "perez",
        "thompson",
        "white",
        "harris",
        "sanchez",
        "clark",
        "ramirez",
        "lewis",
        "robinson",
        "walker",
    ];

    let domains = vec![
        "gmail.com",
        "yahoo.com",
        "outlook.com",
        "hotmail.com",
        "example.com",
        "company.org",
        "business.net",
        "mail.com",
        "proton.me",
        "icloud.com",
        "corporate.io",
        "startup.co",
        "enterprise.biz",
        "service.info",
    ];

    println!("1. Chain-Based Email Generation");
    println!("{}", "-".repeat(50));

    let generator = EmailGenerator::new(
        &first_names.iter().map(|s| s.as_ref()).collect::<Vec<_>>(),
        &domains.iter().map(|s| s.as_ref()).collect::<Vec<_>>(),
    );

    println!("\nGenerated email addresses:");
    for (i, email) in generator.generate_emails(10, 12).iter().enumerate() {
        println!("  {}. {}", i + 1, email);
    }

    println!("\n2. Pattern-Based Email Generation");
    println!("{}", "-".repeat(50));

    let patterns = vec![
        "{name}@{domain}",
        "contact.{name}@{domain}",
        "{name}.support@{domain}",
        "info.{name}@{domain}",
    ];

    for pattern in patterns {
        let email = generator.generate_pattern_email(pattern);
        println!("  Pattern: {} → {}", pattern, email);
    }

    println!("\n3. Simple Pattern-Based Email Generation");
    println!("{}", "-".repeat(50));

    let simple_emails = generate_simple_emails(&first_names, &domains, 10);
    println!("Generated using random patterns:");
    for (i, email) in simple_emails.iter().enumerate() {
        println!("  {}. {}", i + 1, email);
    }

    println!("\n4. Text Model Email Generation");
    println!("{}", "-".repeat(50));

    // Create some realistic email patterns
    let sample_emails = vec![
        "john.smith@gmail.com",
        "jane.johnson@yahoo.com",
        "alice.williams@outlook.com",
        "bob.brown@hotmail.com",
        "charlie.jones@example.com",
        "diana.garcia@company.org",
        "edward.miller@business.net",
        "fiona.davis@mail.com",
        "george.rodriguez@proton.me",
        "helen.martinez@icloud.com",
        "ivan.hernandez@corporate.io",
        "julia.lopez@startup.co",
        "kevin.gonzalez@enterprise.biz",
        "laura.wilson@service.info",
        "michael.anderson@gmail.com",
    ];

    let generated = generate_emails_with_text(&sample_emails, 8);
    println!("Generated from email patterns:");
    for (i, email) in generated.iter().enumerate() {
        println!("  {}. {}", i + 1, email);
    }

    println!("\n5. Username Variations");
    println!("{}", "-".repeat(50));

    // Generate username variations
    let base_name = "johnsmith";
    let variations = vec![
        format!("{}", base_name),
        format!("{}123", base_name),
        format!("{}.official", base_name),
        format!("{}2024", base_name),
        format!("the.{}", base_name),
        format!("{}.real", base_name),
        format!("{}_", base_name),
        format!("_{}_", base_name),
    ];

    println!("Username variations for '{}':", base_name);
    for variation in variations {
        let domain = &domains[0];
        println!("  {}@{}", variation, domain);
    }

    println!("\n6. Domain-Specific Generation");
    println!("{}", "-".repeat(50));

    // Generate for specific domains
    let specific_domains = vec!["tech.corp", "dev.io", "code.lab", "hack.space"];

    for domain in specific_domains {
        let name = generator.generate_name(10);
        println!("  {}@{}", name, domain);
    }

    println!("\n7. Bulk Generation (Test Data)");
    println!("{}", "-".repeat(50));

    let bulk_emails = generator.generate_emails(15, 10);
    println!("Generated {} test email addresses:", bulk_emails.len());
    for (i, email) in bulk_emails.iter().enumerate() {
        println!("  {:2}. {}", i + 1, email);
    }

    println!("\n=== Example Complete ===");
    println!("\nTips for email generation:");
    println!("  • Use character-level chains for more variety");
    println!("  • Combine with validation for production use");
    println!("  • Add numbers/special chars for uniqueness");
    println!("  • Consider GDPR when generating test data");
}
