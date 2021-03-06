use crate::{is_owner, post_bin, Error, PrefixContext};
use execute::{command, Execute};
use std::process::Stdio;

#[poise::command(
    prefix_command,
    track_edits,
    check = "is_owner",
    hide_in_help,
    aliases("sh", "bash", "$")
)]
pub async fn exec(ctx: PrefixContext<'_>, #[rest] code: String) -> Result<(), Error> {
    let mut command = command(code);

    command.stdout(Stdio::piped());

    let output = String::from_utf8(command.execute_output()?.stdout)?;

    if output.chars().count() > 1500 {
        let url = post_bin(ctx, output).await?;
        let msg = format!("<{}>", url);

        poise::send_prefix_reply(ctx, |m| m.content(msg)).await?;
        Ok(())
    } else {
        let msg = format!(
            "```sh
{}
```
        ",
            output
        );

        poise::send_prefix_reply(ctx, |m| m.content(msg)).await?;
        Ok(())
    }
}
