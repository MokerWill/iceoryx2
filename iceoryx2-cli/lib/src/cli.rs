// Copyright (c) 2024 Contributors to the Eclipse Foundation
//
// See the NOTICE file(s) distributed with this work for additional
// information regarding copyright ownership.
//
// This program and the accompanying materials are made available under the
// terms of the Apache Software License 2.0 which is available at
// https://www.apache.org/licenses/LICENSE-2.0, or the MIT license
// which is available at https://opensource.org/licenses/MIT.
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use colored::*;

pub fn help_template(cli_name: &str, show_external_commands: bool) -> String {
    let mut template = format!(
        "{{about}}\n\n{}{}{}[OPTIONS] [COMMAND]\n\n{}\n{{options}}",
        "Usage: ".bright_green().bold(),
        cli_name.bold(),
        " ".bold(),
        "Options:".bright_green().bold(),
    );

    if show_external_commands {
        template.push_str(&format!(
            "\n\n{}\n{{subcommands}}\n{}{}",
            "Commands:".bright_green().bold(),
            "  ...            ".bold(),
            "See external installed commands with --list"
        ));
    } else {
        // Only add subcommands section without the "Commands:" header
        template.push_str("\n\n{{subcommands}}");
    }

    template
}
