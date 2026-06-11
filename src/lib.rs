use zed_extension_api::{
    self as zed, Range, SlashCommand, SlashCommandOutput, SlashCommandOutputSection,
};

mod json_tools;

struct JsonToolsExtension;

impl zed::Extension for JsonToolsExtension {
    fn new() -> Self {
        Self
    }

    fn run_slash_command(
        &self,
        command: SlashCommand,
        args: Vec<String>,
        _worktree: Option<&zed::Worktree>,
    ) -> zed::Result<SlashCommandOutput, String> {
        let input = args.join(" ");
        let text = match command.name.as_str() {
            "json-validate" => {
                if json_tools::is_valid(&input) {
                    "Valid JSON".to_string()
                } else {
                    "Invalid JSON".to_string()
                }
            }
            "json-beautify" => json_tools::beautify(&input, 2),
            "json-uglify" => json_tools::uglify(&input),
            "json-escape" => json_tools::escape(&input),
            "json-unescape" => json_tools::unescape(&input),
            other => return Err(format!("unknown command: {other}")),
        };

        let end = text.len() as u32;
        Ok(SlashCommandOutput {
            text,
            sections: vec![SlashCommandOutputSection {
                range: Range { start: 0, end },
                label: command.name,
            }],
        })
    }
}

zed::register_extension!(JsonToolsExtension);
