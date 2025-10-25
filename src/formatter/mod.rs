use crate::wakatime::stats::StatsData;

const BAR_LENGTH: usize = 25;

pub struct StatsFormatter;

impl StatsFormatter {
    pub fn format(stats: &StatsData) -> String {
        let mut output = String::new();

        // Total time
        let total_time = format_duration(stats.total_seconds);
        output.push_str(&format!("**🕐 Total Coding Time**: {}\n\n", total_time));

        // Programming languages
        if !stats.languages.is_empty() {
            output.push_str("**💻 Programming Languages**:\n```text\n");
            for lang in stats.languages.iter().take(5) {
                let bar = generate_bar(lang.percent);
                output.push_str(&format!(
                    "{:<15} {} {:>6} {}\n",
                    lang.name,
                    bar,
                    format!("{:.2}%", lang.percent),
                    lang.text
                ));
            }
            output.push_str("```\n\n");
        }

        // Editors
        if !stats.editors.is_empty() {
            output.push_str("**🛠️ Editors**:\n```text\n");
            for editor in stats.editors.iter().take(3) {
                let bar = generate_bar(editor.percent);
                output.push_str(&format!(
                    "{:<15} {} {:>6}\n",
                    editor.name,
                    bar,
                    format!("{:.2}%", editor.percent)
                ));
            }
            output.push_str("```\n\n");
        }

        // Operating systems
        if !stats.operating_systems.is_empty() {
            output.push_str("**💻 Operating Systems**:\n```text\n");
            for os in stats.operating_systems.iter().take(3) {
                let bar = generate_bar(os.percent);
                output.push_str(&format!(
                    "{:<15} {} {:>6}\n",
                    os.name,
                    bar,
                    format!("{:.2}%", os.percent)
                ));
            }
            output.push_str("```\n\n");
        }

        // Projects
        if !stats.projects.is_empty() {
            output.push_str("**📂 Projects**:\n```text\n");
            for project in stats.projects.iter().take(5) {
                let bar = generate_bar(project.percent);
                output.push_str(&format!(
                    "{:<30} {} {:>6}\n",
                    project.name,
                    bar,
                    format!("{:.2}%", project.percent)
                ));
            }
            output.push_str("```\n");
        }

        output
    }
}

fn generate_bar(percent: f64) -> String {
    let filled = ((percent / 100.0) * BAR_LENGTH as f64).round() as usize;
    let filled = filled.min(BAR_LENGTH);
    let empty = BAR_LENGTH - filled;

    format!("{}{}", "█".repeat(filled), "░".repeat(empty))
}

fn format_duration(seconds: f64) -> String {
    let hours = (seconds / 3600.0).floor() as u32;
    let minutes = ((seconds % 3600.0) / 60.0).floor() as u32;

    if hours > 0 {
        format!("{} hrs {} mins", hours, minutes)
    } else {
        format!("{} mins", minutes)
    }
}
