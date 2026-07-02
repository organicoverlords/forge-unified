#![allow(dead_code)]

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "graphify")]
#[command(about = "Knowledge graph extraction and visualization for codebases")]
#[command(version = "2.0.0")]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    Extract(ExtractArgs),
    Query(QueryArgs),
    Serve(ServeArgs),
    Export(ExportArgs),
    Install(InstallArgs),
    Uninstall(UninstallArgs),
    Status(StatusArgs),
    Hook(HookArgs),
    Watch(WatchArgs),
    Update(UpdateArgs),
    ClaudeInstall(PlatformInstallArgs),
    CodexInstall(PlatformInstallArgs),
    OpenCodeInstall(PlatformInstallArgs),
    CursorInstall(PlatformInstallArgs),
    GeminiInstall(PlatformInstallArgs),
    AiderInstall(PlatformInstallArgs),
    TraeInstall(PlatformInstallArgs),
    TraeCNInstall(PlatformInstallArgs),
    FactoryDroidInstall(PlatformInstallArgs),
    KiloInstall(PlatformInstallArgs),
    AgentsInstall(PlatformInstallArgs),
    Path(PathArgs),
    Explain(ExplainArgs),
    History(HistoryArgs),
    Cluster(ClusterArgs),
    Analytics(AnalyticsArgs),
    Centrality(CentralityArgs),
    Community(CommunityArgs),
    Insights(InsightsArgs),
    GraphML(GraphMLArgs),
    SVG(SVGAArgs),
    Obsidian(ObsidianArgs),
    Wiki(WikiArgs),
    Markdown(MarkdownArgs),
    Neo4j(Neo4jArgs),
    Postgres(PostgresArgs),
    GoogleWorkspace(GoogleWorkspaceArgs),
    PDF(PDFAArgs),
    Office(OfficeArgs),
    Video(VideoArgs),
    SQL(SQLArgs),
    Apache(ApacheArgs),
    Terraform(TerraformArgs),
    DMG(DMGArgs),
    Chinese(ChineseArgs),
    PRS(PRArgs),
    MergeGraphs(MergeArgs),
    Add(AddArgs),
    Triage(TriageArgs),
    Conflicts(ConflictArgs),
    Config(ConfigArgs),
    Profile(ProfileArgs),
    Settings(SettingsArgs),
    MCP(MCPArgs),
    ClusterOnly(ClusterOnlyArgs),
    NoViz(NoVizArgs),
    ObsidianMode(ObsidianModeArgs),
    WatchOnly(WatchOnlyArgs),
    UpdateOnly(UpdateOnlyArgs),
}

#[derive(Args)]
pub(crate) struct ExtractArgs {
    #[arg(required = true, value_name = "PATHS")]
    pub(crate) paths: Vec<String>,
    #[arg(short, long, value_name = "FILE")]
    pub(crate) output: Option<String>,
    #[arg(short = 'e', long)]
    pub(crate) with_edges: bool,
    #[arg(short = 'm', long, default_value = "1")]
    pub(crate) max_depth: u32,
    #[arg(short = 'l', long, default_value = "deep")]
    pub(crate) mode: String,
    #[arg(short = 'f', long, default_value = ".graphifyignore")]
    pub(crate) ignore_file: String,
    #[arg(long)]
    pub(crate) force: bool,
    #[arg(long)]
    pub(crate) update_only: bool,
    #[arg(long)]
    pub(crate) cluster_only: bool,
    #[arg(long)]
    pub(crate) no_viz: bool,
}

#[derive(Args)]
pub(crate) struct QueryArgs {
    #[arg(required = true)]
    pub(crate) pattern: String,
    #[arg(short, long, value_name = "TYPE")]
    pub(crate) node_type: Option<String>,
    #[arg(short, long, value_name = "PATH")]
    pub(crate) file: Option<String>,
    #[arg(short, long)]
    pub(crate) json: bool,
    #[arg(short, long)]
    pub(crate) dfs: bool,
    #[arg(short, long, default_value = "1500")]
    pub(crate) budget: u32,
    #[arg(long)]
    pub(crate) graph: Option<String>,
}

#[derive(Args)]
pub(crate) struct ServeArgs {
    #[arg(short, long, default_value = "127.0.0.1")]
    pub(crate) host: String,
    #[arg(short, long, default_value = "8080")]
    pub(crate) port: u16,
    #[arg(long)]
    pub(crate) cors: bool,
    #[arg(long)]
    pub(crate) api_key: Option<String>,
    #[arg(long, default_value = "/mcp")]
    pub(crate) path: String,
    #[arg(long)]
    pub(crate) json_response: bool,
    #[arg(long)]
    pub(crate) stateless: bool,
    #[arg(long, default_value = "3600")]
    pub(crate) session_timeout: u64,
}

#[derive(Args)]
pub(crate) struct ExportArgs {
    #[arg(short, long, value_name = "FORMAT", default_value = "json")]
    pub(crate) format: String,
    #[arg(short, long, value_name = "FILE")]
    pub(crate) output: Option<String>,
    #[arg(long)]
    pub(crate) graph: Option<String>,
    #[arg(long)]
    pub(crate) obsidian: bool,
    #[arg(long)]
    pub(crate) wiki: bool,
    #[arg(long)]
    pub(crate) svg: bool,
    #[arg(long)]
    pub(crate) graphml: bool,
    #[arg(long)]
    pub(crate) neo4j: bool,
    #[arg(long)]
    pub(crate) cypher_only: bool,
    #[arg(long)]
    pub(crate) markdown: bool,
    #[arg(long)]
    pub(crate) callflow_html: bool,
}

#[derive(Args)]
pub(crate) struct InstallArgs { #[arg(value_name = "TYPE")] pub(crate) hook_type: String }
#[derive(Args)]
pub(crate) struct PlatformInstallArgs { #[arg(short, long)] pub(crate) project: bool }
#[derive(Args)]
pub(crate) struct UninstallArgs { #[arg(short, long)] pub(crate) purge: bool, #[arg(value_name = "TYPE")] pub(crate) platform: Option<String> }
#[derive(Args)]
pub(crate) struct StatusArgs { #[arg(long)] pub(crate) verbose: bool }
#[derive(Args)]
pub(crate) struct HookArgs { #[arg(value_name = "ACTION")] pub(crate) action: String }
#[derive(Args)]
pub(crate) struct WatchArgs { #[arg(short, long, value_name = "INTERVAL")] pub(crate) interval: Option<u64>, #[arg(short, long)] pub(crate) ignore: bool }
#[derive(Args)]
pub(crate) struct UpdateArgs { #[arg(short, long)] pub(crate) files_only: bool, #[arg(short, long)] pub(crate) reextract: bool }

#[derive(Args)]
pub(crate) struct PathArgs { #[arg(required = true)] pub(crate) start: String, #[arg(required = true)] pub(crate) end: String, #[arg(short, long, default_value = "shortest")] pub(crate) algorithm: String, #[arg(long)] pub(crate) graph: Option<String> }
#[derive(Args)]
pub(crate) struct ExplainArgs { #[arg(required = true)] pub(crate) node: String, #[arg(long)] pub(crate) depth: Option<u32>, #[arg(long)] pub(crate) graph: Option<String> }
#[derive(Args)]
pub(crate) struct HistoryArgs { #[arg(long, default_value = "100")] pub(crate) limit: usize, #[arg(long)] pub(crate) format: Option<String> }
#[derive(Args)]
pub(crate) struct ClusterArgs { #[arg(short, long, default_value = "leiden")] pub(crate) algorithm: String, #[arg(short, long, default_value = "1.0")] pub(crate) resolution: f64, #[arg(long)] pub(crate) exclude_hubs: bool, #[arg(long)] pub(crate) min_size: usize, #[arg(long)] pub(crate) graph: Option<String> }
#[derive(Args)]
pub(crate) struct AnalyticsArgs { #[arg(short, long, value_name = "TYPE")] pub(crate) analytics_type: String, #[arg(long)] pub(crate) output_format: Option<String>, #[arg(long)] pub(crate) graph: Option<String> }
#[derive(Args)]
pub(crate) struct CentralityArgs { #[arg(short, long, value_name = "TYPE")] pub(crate) centrality_type: String, #[arg(long)] pub(crate) top_n: Option<usize> }
#[derive(Args)]
pub(crate) struct CommunityArgs { #[arg(long)] pub(crate) min_size: Option<usize>, #[arg(long)] pub(crate) algorithm: Option<String> }
#[derive(Args)]
pub(crate) struct InsightsArgs { #[arg(long)] pub(crate) insight_type: Option<String>, #[arg(long)] pub(crate) confidence_threshold: Option<f64>, #[arg(long)] pub(crate) graph: Option<String> }
#[derive(Args)]
pub(crate) struct GraphMLArgs { #[arg(short, long, value_name = "FILE")] pub(crate) output: Option<String>, #[arg(long)] pub(crate) include_metadata: bool }
#[derive(Args)]
pub(crate) struct SVGAArgs { #[arg(short, long, value_name = "FILE")] pub(crate) output: Option<String>, #[arg(long)] pub(crate) width: Option<u32>, #[arg(long)] pub(crate) height: Option<u32> }
#[derive(Args)]
pub(crate) struct ObsidianArgs { #[arg(short, long, value_name = "DIR")] pub(crate) directory: Option<String> }
#[derive(Args)]
pub(crate) struct WikiArgs { #[arg(short, long, value_name = "DIR")] pub(crate) directory: Option<String> }
#[derive(Args)]
pub(crate) struct MarkdownArgs { #[arg(short, long, value_name = "FILE")] pub(crate) output: Option<String> }

#[derive(Args)]
pub(crate) struct Neo4jArgs { #[arg(short, long, value_name = "URI")] pub(crate) uri: Option<String>, #[arg(short, long, value_name = "USER")] pub(crate) user: Option<String>, #[arg(short, long, value_name = "PASSWORD")] pub(crate) password: Option<String>, #[arg(long)] pub(crate) cypher_only: bool, #[arg(short, long, value_name = "FILE")] pub(crate) output: Option<String> }
#[derive(Args)]
pub(crate) struct PostgresArgs { #[arg(short, long, value_name = "DSN")] pub(crate) dsn: Option<String> }
#[derive(Args)]
pub(crate) struct GoogleWorkspaceArgs { #[arg(short, long)] pub(crate) enable: bool }
#[derive(Args)]
pub(crate) struct PDFAArgs { #[arg(short, long)] pub(crate) enable: bool }
#[derive(Args)]
pub(crate) struct OfficeArgs { #[arg(short, long)] pub(crate) enable: bool }
#[derive(Args)]
pub(crate) struct VideoArgs { #[arg(short, long)] pub(crate) enable: bool }
#[derive(Args)]
pub(crate) struct SQLArgs { #[arg(short, long, value_name = "DSN")] pub(crate) dsn: Option<String> }
#[derive(Args)]
pub(crate) struct ApacheArgs { #[arg(short, long)] pub(crate) enable: bool }
#[derive(Args)]
pub(crate) struct TerraformArgs { #[arg(short, long)] pub(crate) enable: bool }
#[derive(Args)]
pub(crate) struct DMGArgs { #[arg(short, long)] pub(crate) enable: bool }
#[derive(Args)]
pub(crate) struct ChineseArgs { #[arg(short, long)] pub(crate) enable: bool }

#[derive(Args)]
pub(crate) struct PRArgs { #[arg(value_name = "NUMBER")] pub(crate) number: Option<i32>, #[arg(long)] pub(crate) triage: bool, #[arg(long)] pub(crate) conflicts: bool }
#[derive(Args)]
pub(crate) struct MergeArgs { #[arg(required = true, value_name = "FILES")] pub(crate) files: Vec<String>, #[arg(short, long, value_name = "FILE")] pub(crate) output: Option<String> }
#[derive(Args)]
pub(crate) struct AddArgs { #[arg(required = true)] pub(crate) source: String, #[arg(short, long, value_name = "AUTHOR")] pub(crate) author: Option<String>, #[arg(short, long, value_name = "CONTRIBUTOR")] pub(crate) contributor: Option<String> }
#[derive(Args)]
pub(crate) struct TriageArgs { #[arg(long)] pub(crate) auto_ranking: bool, #[arg(long)] pub(crate) confidence_threshold: Option<f64> }
#[derive(Args)]
pub(crate) struct ConflictArgs { #[arg(long)] pub(crate) communities: bool, #[arg(long)] pub(crate) merge_order_risk: bool }

#[derive(Args)]
pub(crate) struct ConfigArgs { #[arg(value_name = "KEY")] pub(crate) key: Option<String>, #[arg(value_name = "VALUE")] pub(crate) value: Option<String>, #[arg(long)] pub(crate) list: bool }
#[derive(Args)]
pub(crate) struct ProfileArgs { #[arg(value_name = "NAME")] pub(crate) name: String, #[arg(short, long, value_name = "FILE")] pub(crate) config: Option<String> }
#[derive(Args)]
pub(crate) struct SettingsArgs { #[arg(long)] pub(crate) show_defaults: bool }
#[derive(Args)]
pub(crate) struct MCPArgs { #[arg(short, long, value_name = "TRANSPORT")] pub(crate) transport: String, #[arg(long)] pub(crate) host: Option<String>, #[arg(long)] pub(crate) port: Option<u16> }
#[derive(Args)]
pub(crate) struct ClusterOnlyArgs { #[arg(long)] pub(crate) resolution: Option<f64>, #[arg(long)] pub(crate) exclude_hubs: bool }
#[derive(Args)]
pub(crate) struct NoVizArgs {}
#[derive(Args)]
pub(crate) struct ObsidianModeArgs {}
#[derive(Args)]
pub(crate) struct WatchOnlyArgs { #[arg(short, long, value_name = "INTERVAL")] pub(crate) interval: Option<u64> }
#[derive(Args)]
pub(crate) struct UpdateOnlyArgs { #[arg(short, long)] pub(crate) files_only: bool }
