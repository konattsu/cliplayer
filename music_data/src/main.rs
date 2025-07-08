use clap::Parser;

fn main() {
    let cli = musictl::cli::Cli::parse();
}

//     match cli.command {
//         musictl::cli::Commands::Apply(apply_cmd) => match apply_cmd {
//             musictl::cli::ApplyCommands::New {
//                 input,
//                 common_opts,
//                 output_dir,
//                 output_min_file,
//             } => {
//                 todo!()
//             }
//             musictl::cli::ApplyCommands::Update {
//                 input,
//                 common_opts,
//                 output_dir,
//                 output_min_file,
//             } => {
//                 todo!()
//             }
//         },
//         musictl::cli::Commands::Validate(validate_cmd) => match validate_cmd {
//             musictl::cli::ValidateCommands::NewInput { input, common_opts } => {
//                 todo!()
//             }
//             musictl::cli::ValidateCommands::UpdateInput { input, common_opts } => {
//                 todo!()
//             }
//             musictl::cli::ValidateCommands::Duplicate { input, common_opts } => {
//                 todo!()
//             }
//         },
//     }
// }

// fn apply_new_process(
//     input: musictl::cli::GlobPattern,
//     common_opts: musictl::cli::CommonOpts,
// ) {
//     todo!()
// }

// fn apply_update_process(
//     input: musictl::cli::GlobPattern,
//     common_opts: musictl::cli::CommonOpts,
// ) {
//     todo!()
// }

// fn validate_new_process(
//     input: musictl::cli::GlobPattern,
//     common_opts: musictl::cli::CommonOpts,
// ) {
//     todo!()
// }

// fn validate_update_process(
//     input: musictl::cli::GlobPattern,
//     common_opts: musictl::cli::CommonOpts,
// ) {
//     todo!()
// }

// fn validate_duplicate_process(
//     input: musictl::cli::VideoIds,
//     common_opts: musictl::cli::CommonOpts,
// ) {
//     todo!()
// }

// fn output_min() {
//     todo!()
// }
