/*
Copyright (c) 2023 Ade M Ramdani <qcynaut@gmail.com>

This software is proprietary and licensed to MyRTS under the terms of the Closed-Source Software License for Freelancers, which is available at https://dictionary.cambridge.org/us/dictionary/english/license.

MyRTS owns all right, title, and interest in and to the software, including all intellectual property rights therein.
MyRTS may use the software for any purpose, including commercial use.
MyRTS may modify the software, but only for their own internal use.
MyRTS may not distribute the software or any modified versions of the software to third parties.
MyRTS may not reverse engineer the software.
MyRTS may not create derivative works from the software.

MyRTS agrees to credit you as the developer of the software in all promotional materials and documentation for the software.

If MyRTS violates any of these terms, their license to use the software will automatically terminate.
*/

use proto::{
    app::{MsgData, Stream},
    error::Result,
};
use std::process::Command;
use types::proto::{CmdRequest, CmdResponse};

#[proto::service("command")]
async fn command(stream: Stream, data: MsgData<CmdRequest>) -> Result<()> {
    let data = data.into_inner();
    if data.command.contains("&&") {
        let commands = data.command.split("&&").collect::<Vec<&str>>();
        if commands.len() < 1 {
            return Ok(());
        }
        let cmd = commands[0];
        let cmds = cmd.split(" ").collect::<Vec<&str>>();
        let procs = Command::new(cmds[0]).args(&cmds[1..]).output();
        #[allow(unused_assignments)]
        let mut res = String::new();
        if let Ok(output) = procs {
            if !output.status.success() {
                res = String::from_utf8_lossy(&output.stderr).to_string();
            } else {
                res = String::from_utf8_lossy(&output.stdout).to_string();
            }
        } else {
            stream
                .write(
                    "command",
                    CmdResponse {
                        sender: data.sender,
                        response: "Failed to execute command".to_string(),
                        target: data.target,
                    },
                )
                .await?;
            return Ok(());
        }

        for cmd in &commands[1..] {
            let cmds = cmd.split(" ").collect::<Vec<&str>>();
            let procs = Command::new(cmds[0]).args(&cmds[1..]).output();
            if let Ok(output) = procs {
                if !output.status.success() {
                    res.push_str(format!("\n{}", String::from_utf8_lossy(&output.stderr)).as_str());
                } else {
                    res.push_str(format!("\n{}", String::from_utf8_lossy(&output.stdout)).as_str());
                }
            } else {
                stream
                    .write(
                        "command",
                        CmdResponse {
                            sender: data.sender,
                            response: format!("{}\nFailed to execute command", res),
                            target: data.target,
                        },
                    )
                    .await?;
                return Ok(());
            }
        }

        let response = CmdResponse {
            sender: data.sender,
            response: res,
            target: data.target,
        };

        stream.write("command", response).await?;
    } else {
        let cmds = data.command.split(" ").collect::<Vec<&str>>();
        let procs = Command::new(cmds[0]).args(&cmds[1..]).output();

        if let Ok(output) = procs {
            if !output.status.success() {
                let response = CmdResponse {
                    sender: data.sender,
                    response: String::from_utf8_lossy(&output.stderr).to_string(),
                    target: data.target,
                };
                stream.write("command", response).await?;
            } else {
                let response = CmdResponse {
                    sender: data.sender,
                    response: String::from_utf8_lossy(&output.stdout).to_string(),
                    target: data.target,
                };
                stream.write("command", response).await?;
            }
        } else {
            stream
                .write(
                    "command",
                    CmdResponse {
                        sender: data.sender,
                        response: "Failed to execute command".to_string(),
                        target: data.target,
                    },
                )
                .await?;
        }
    }

    Ok(())
}
