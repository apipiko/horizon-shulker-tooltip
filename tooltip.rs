// Shulker Box Tooltip for Horizon (text version).
// Shows EVERYTHING inside a shulker box in its tooltip, totals per item,
// biggest stack first. Original code; inspired by MisterPeModder's
// ShulkerBoxTooltip (MIT).
fn tooltip(stack: &ItemStack, lines: &mut Vec<(Text, u32)>) {
    let id: &str = &stack.item;
    let path = id.trim_start_matches("minecraft:");
    if !path.ends_with("shulker_box") {
        return;
    }
    let Some(data) = stack.data() else {
        return;
    };
    if data.container.is_empty() {
        return;
    }

    // Total count of each item over all slots.
    let mut totals: Vec<(String, u32)> = Vec::new();
    for (_, s) in data.container.iter() {
        let key: String = s.item.to_string();
        let mut found = false;
        for entry in totals.iter_mut() {
            if entry.0 == key {
                entry.1 += s.count;
                found = true;
                break;
            }
        }
        if !found {
            totals.push((key, s.count));
        }
    }
    totals.sort_by(|a, b| b.1.cmp(&a.1));

    // Take out vanilla's own short list ("3 x Stone", "and 2 more...").
    let mut i = 1;
    while i < lines.len() {
        let t = lines[i].0.plain();
        let counted = t.contains(" x ")
            && t.split(" x ")
                .next()
                .map_or(false, |n| !n.is_empty() && n.chars().all(|c| c.is_ascii_digit()));
        let more = t.starts_with("and ") && t.ends_with("...");
        if counted || more {
            lines.remove(i);
        } else {
            i += 1;
        }
    }

    // Our full list, right under the item's name.
    let gray = rgba(170, 170, 170, 255);
    let at = 1.min(lines.len());
    let mut k = 0;
    for (item, n) in totals.iter() {
        let short = item.rsplit(':').next().unwrap_or(item.as_str());
        let mut name = String::new();
        let mut up = true;
        for c in short.chars() {
            if c == '_' || c == '/' {
                name.push(' ');
                up = true;
            } else if up {
                for u in c.to_uppercase() {
                    name.push(u);
                }
                up = false;
            } else {
                name.push(c);
            }
        }
        let line = n.to_string() + "x " + &name;
        lines.insert(at + k, (Text::literal(line), gray));
        k += 1;
    }
}
