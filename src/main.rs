use anyhow::{anyhow, Result};
use async_trait::async_trait;
use serde::Deserialize;
use std::sync::Arc;
use zed_extension_api::{self as zed, ActionContext, Extension, ExtensionContext, SlashCommand};

#[derive(Default)]
struct PrefixJumpExtension {
    context: Option<ExtensionContext>,
}

#[derive(Debug, Deserialize)]
struct CommandParams {
    argument: String,
}

struct PrefixJumpCommand;

#[async_trait(?Send)]
impl SlashCommand for PrefixJumpCommand {
    fn name(&self) -> &'static str {
        "prefix-jump"
    }

    fn description(&self) -> &'static str {
        "Jump to a location based on a prefix"
    }

    async fn run(&self, cx: ActionContext, params_json: String) -> Result<()> {
        // Parse the command parameters
        let params: CommandParams = serde_json::from_str(&params_json)
            .map_err(|e| anyhow!("Failed to parse command parameters: {}", e))?;
        
        // Get the active editor
        let editor = cx.active_editor().ok_or_else(|| anyhow!("No active editor"))?;
        
        // Example: Log the command execution
        log::info!("Executing prefix-jump command with argument: {}", params.argument);
        
        // TODO: Implement the actual prefix jump functionality
        // This would involve:
        // 1. Searching the document for the prefix
        // 2. Moving the cursor to the matching location
        // 3. Possibly highlighting the match
        
        // Example notification to show command was received
        cx.show_notification(
            "Prefix Jump",
            &format!("Command executed with: {}", params.argument),
            None,
        );
        
        Ok(())
    }
}

#[async_trait(?Send)]
impl Extension for PrefixJumpExtension {
    fn name(&self) -> &str {
        "prefix-jump"
    }

    fn activate(&mut self, context: ExtensionContext) -> Result<()> {
        log::info!("Activating prefix-jump extension");
        
        // Register the slash command
        context.register_slash_command(Arc::new(PrefixJumpCommand))?;
        
        // Store context for later use
        self.context = Some(context);
        
        Ok(())
    }

    fn deactivate(&mut self) -> Result<()> {
        log::info!("Deactivating prefix-jump extension");
        self.context = None;
        Ok(())
    }
}

zed::register_extension!(PrefixJumpExtension::default());

