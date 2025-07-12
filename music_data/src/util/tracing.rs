/// トレースログの設定を行う
///
/// - `stdout_level`: 標準出力のログレベル
/// - `file_level`: ファイル出力のログレベル
///
/// 戻り値が生きている間は、ログ出力が有効になる。
pub fn apply_tracing_settings(
    stdout_level: Option<tracing::level_filters::LevelFilter>,
    file_level: Option<tracing::level_filters::LevelFilter>,
) -> tracing_appender::non_blocking::WorkerGuard {
    use tracing_subscriber::Layer;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    let stdout_layer = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stdout)
        .event_format(FormatterForStdout)
        .with_filter(filter_level(stdout_level));

    let file_appender =
        tracing_appender::rolling::daily("./logs", "rust_music_data.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    let file_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_writer(non_blocking)
        .with_filter(filter_level(file_level));

    tracing_subscriber::registry()
        .with(stdout_layer)
        .with(file_layer)
        .init();
    guard
}

/// ログ出力のフィルタを指定
///
/// - `None`: ログを出力しない
fn filter_level(
    level: Option<tracing::level_filters::LevelFilter>,
) -> tracing_subscriber::EnvFilter {
    use tracing_subscriber::EnvFilter;

    match level.and_then(|lv| lv.into_level()) {
        Some(level) => EnvFilter::from(level.as_str()),
        None => {
            // details of constant, ref:
            // https://docs.rs/tracing-subscriber/0.3.18/src/tracing_subscriber/filter/env/directive.rs.html#125-139
            // https://docs.rs/tracing-core/0.1.32/src/tracing_core/metadata.rs.html#776-802
            const NO_OUTPUT: &str = "off";
            EnvFilter::new(NO_OUTPUT)
        }
    }
}

struct FormatterForStdout;

// ref:
// https://docs.rs/tracing-subscriber/latest/tracing_subscriber/fmt/trait.FormatEvent.html

impl<S, N> tracing_subscriber::fmt::FormatEvent<S, N> for FormatterForStdout
where
    S: tracing::Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
    N: for<'a> tracing_subscriber::fmt::FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &tracing_subscriber::fmt::FmtContext<'_, S, N>,
        mut writer: tracing_subscriber::fmt::format::Writer<'_>,
        event: &tracing::Event<'_>,
    ) -> std::fmt::Result {
        // Format values from the event's's metadata:
        let metadata = event.metadata();
        // write!(&mut writer, "{} {}: ", metadata.level(), metadata.target())?;

        write!(
            &mut writer,
            "{:<5} [{}:ln{}] ",
            metadata.level(),
            metadata.target(),
            metadata.line().unwrap_or_default()
        )?;

        if let Some(scope) = ctx.event_scope() {
            for span in scope.from_root() {
                write!(writer, "{}", span.name())?;

                // `FormattedFields` is a formatted representation of the span's
                // fields, which is stored in its extensions by the `fmt` layer's
                // `new_span` method. The fields will have been formatted
                // by the same field formatter that's provided to the event
                // formatter in the `FmtContext`.
                let ext = span.extensions();
                let fields = &ext
                    .get::<tracing_subscriber::fmt::FormattedFields<N>>()
                    .expect("will never be `None`");

                // Skip formatting the fields if the span had no fields.
                if !fields.is_empty() {
                    write!(writer, "{{{fields}}}")?;
                }
                write!(writer, ": ")?;
            }
        }

        // Write fields on the event
        ctx.field_format().format_fields(writer.by_ref(), event)?;

        writeln!(writer)
    }
}
