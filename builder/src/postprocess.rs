use regex::Regex;
use std::collections::HashMap;
use std::sync::LazyLock;

/// Regex pattern to match item markup: <item name="item-id">display text</item>
static ITEM_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"<item\s+name="([^"]+)"[^>]*>([^<]*)</item>"#).unwrap()
});

/// Regex pattern to match NPC markup: <npc name="npc-id">display text</npc>
static NPC_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"<npc\s+name="([^"]+)"[^>]*>([^<]*)</npc>"#).unwrap()
});

/// Generate an item link with icon
fn item_link(item_id: &str, display_text: &str) -> String {
    format!(
        r#"<a href="/items/{}.html" class="item-link"><img src="/assets/images/items/{}.png" alt="{}" class="inline-icon" />{}</a>"#,
        item_id, item_id, display_text, display_text
    )
}

/// Generate an NPC link with icon
fn npc_link(npc_id: &str, display_text: &str) -> String {
    format!(
        r#"<a href="/npcs/{}.html" class="npc-link"><img src="/assets/images/npcs/{}.png" alt="{}" class="inline-icon" />{}</a>"#,
        npc_id, npc_id, display_text, display_text
    )
}

/// Post-process text to convert item and NPC markup into links with icons.
pub fn linkify_references(text: &str) -> String {
    let mut result = text.to_string();

    result = ITEM_PATTERN
        .replace_all(&result, |caps: &regex::Captures| {
            let item_id = &caps[1];
            let display_text = &caps[2];
            item_link(item_id, display_text)
        })
        .to_string();

    result = NPC_PATTERN
        .replace_all(&result, |caps: &regex::Captures| {
            let npc_id = &caps[1];
            let display_text = &caps[2];
            npc_link(npc_id, display_text)
        })
        .to_string();

    result
}

/// Create a Tera filter function for linkifying
pub fn make_linkify_filter() -> impl tera::Filter {
    |value: &tera::Value, _args: &HashMap<String, tera::Value>| -> tera::Result<tera::Value> {
        match value.as_str() {
            Some(s) => Ok(tera::Value::String(linkify_references(s))),
            None => Ok(value.clone()),
        }
    }
}

/// Generate an item type link
fn item_type_link(item_type: &str) -> String {
    let slug = item_type.to_lowercase().replace(' ', "-");
    format!(
        r#"<a href="/items/?type={}" class="type-link">{}</a>"#,
        slug, item_type
    )
}

/// Create a Tera filter function for linking item types
pub fn make_item_type_link_filter() -> impl tera::Filter {
    |value: &tera::Value, _args: &HashMap<String, tera::Value>| -> tera::Result<tera::Value> {
        match value.as_str() {
            Some(s) => Ok(tera::Value::String(item_type_link(s))),
            None => Ok(value.clone()),
        }
    }
}
