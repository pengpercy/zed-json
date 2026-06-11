use zed_extension_api::{
    self as zed, Range, SlashCommand, SlashCommandOutput, SlashCommandOutputSection,
};

mod json_tools;

struct JsonToolsExtension;

fn slash_output(command: SlashCommand, text: String) -> SlashCommandOutput {
    let end = text.len() as u32;
    SlashCommandOutput {
        text,
        sections: vec![SlashCommandOutputSection {
            range: Range { start: 0, end },
            label: command.name,
        }],
    }
}

fn beautify_args(args: Vec<String>) -> (String, String) {
    let mut indent = "  ".to_string();
    let mut start = 0;

    match args.as_slice() {
        [flag, value, ..] if flag == "--indent" => {
            if let Ok(width) = value.parse::<usize>() {
                indent = " ".repeat(width.clamp(1, 16));
                start = 2;
            }
        }
        [flag, ..] if flag == "--tabs" => {
            indent = "\t".to_string();
            start = 1;
        }
        _ => {}
    }

    (args[start..].join(" "), indent)
}

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
        let text = match command.name.as_str() {
            "json-validate" => {
                let input = args.join(" ");
                match json_tools::validate(&input) {
                    Ok(()) => "Valid JSON".to_string(),
                    Err(error) => format!("Invalid JSON: {error}"),
                }
            }
            "json-beautify" => {
                let (input, indent) = beautify_args(args);
                if indent == "  " {
                    json_tools::beautify(&input, 2)
                } else {
                    json_tools::beautify_with_indent(&input, &indent)
                }
            }
            "json-uglify" => json_tools::uglify(&args.join(" ")),
            "json-escape" => json_tools::escape(&args.join(" ")),
            "json-unescape" => json_tools::unescape(&args.join(" ")),
            other => return Err(format!("unknown command: {other}")),
        };

        Ok(slash_output(command, text))
    }
}

zed::register_extension!(JsonToolsExtension);
