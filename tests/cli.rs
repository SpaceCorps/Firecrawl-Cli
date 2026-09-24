//! Drives the built binary against an in-process mock of the Firecrawl API. Every test gets its
//! own config directory and the plaintext store, so nothing touches a real keystore or account.

use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use serde_json::{Value, json};

#[derive(Clone, Debug)]
struct Recorded {
    method: String,
    path: String,
    headers: Vec<(String, String)>,
    body: Option<Value>,
}

type Route = (&'static str, &'static str, u16, Value);

struct Mock {
    url: String,
    log: Arc<Mutex<Vec<Recorded>>>,
}

impl Mock {
    /// Routes are (method, path-without-prefix, status, body). Unmatched requests get a 404.
    fn start(routes: Vec<Route>) -> Mock {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}/v2/", listener.local_addr().unwrap());
        let log = Arc::new(Mutex::new(Vec::new()));
        let log2 = log.clone();
        std::thread::spawn(move || {
            for stream in listener.incoming() {
                let Ok(mut stream) = stream else { continue };
                let routes = routes.clone();
                let log = log2.clone();
                std::thread::spawn(move || {
                    let mut reader = BufReader::new(stream.try_clone().unwrap());
                    let mut line = String::new();
                    if reader.read_line(&mut line).unwrap_or(0) == 0 {
                        return;
                    }
                    let mut parts = line.split_whitespace();
                    let method = parts.next().unwrap_or("").to_string();
                    let raw_path = parts.next().unwrap_or("").to_string();
                    let path = raw_path.trim_start_matches("/v2/").to_string();
                    let mut headers = Vec::new();
                    let mut len = 0usize;
                    loop {
                        let mut h = String::new();
                        reader.read_line(&mut h).unwrap();
                        let h = h.trim_end();
                        if h.is_empty() {
                            break;
                        }
                        if let Some((k, v)) = h.split_once(':') {
                            let (k, v) = (k.trim().to_lowercase(), v.trim().to_string());
                            if k == "content-length" {
                                len = v.parse().unwrap_or(0);
                            }
                            headers.push((k, v));
                        }
                    }
                    let mut buf = vec![0; len];
                    reader.read_exact(&mut buf).unwrap();
                    let body = (len > 0).then(|| serde_json::from_slice(&buf).unwrap());
                    log.lock().unwrap().push(Recorded { method: method.clone(), path: path.clone(), headers, body });

                    let (status, resp) = routes
                        .iter()
                        .find(|(m, p, _, _)| *m == method && *p == path)
                        .map(|(_, _, s, b)| (*s, b.clone()))
                        .unwrap_or((404, json!({"code": "not_found", "error": "no route"})));
                    let text = if status == 204 { String::new() } else { resp.to_string() };
                    let _ = write!(
                        stream,
                        "HTTP/1.1 {status} X\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{text}",
                        text.len()
                    );
                });
            }
        });
        Mock { url, log }
    }

    fn requests(&self) -> Vec<Recorded> {
        self.log.lock().unwrap().clone()
    }

    fn last(&self, method: &str) -> Recorded {
        self.requests().into_iter().rev().find(|r| r.method == method).expect("no such request")
    }
}

struct Env {
    dir: PathBuf,
    api: String,
}

impl Env {
    fn new(mock: &Mock) -> Env {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let dir = std::env::temp_dir().join(format!(
            "firecrawl-test-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir_all(&dir).unwrap();
        Env { dir, api: mock.url.clone() }
    }

    fn run(&self, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_firecrawl"))
            .args(args)
            .env("FIRECRAWL_CONFIG_DIR", &self.dir)
            .env("FIRECRAWL_SECRET_STORE", "plaintext")
            .env("FIRECRAWL_ALLOW_PLAINTEXT_STORE", "1")
            .env("FIRECRAWL_API_URL", &self.api)
            .env_remove("FIRECRAWL_API_KEY")
            .output()
            .unwrap()
    }

    fn run_with_env(&self, args: &[&str], key: &str, val: &str) -> Output {
        Command::new(env!("CARGO_BIN_EXE_firecrawl"))
            .args(args)
            .env("FIRECRAWL_CONFIG_DIR", &self.dir)
            .env("FIRECRAWL_SECRET_STORE", "plaintext")
            .env("FIRECRAWL_ALLOW_PLAINTEXT_STORE", "1")
            .env("FIRECRAWL_API_URL", &self.api)
            .env(key, val)
            .output()
            .unwrap()
    }

    fn json(&self, args: &[&str]) -> (i32, Value, Value) {
        let mut all = args.to_vec();
        all.push("--json");
        let out = self.run(&all);
        let parse = |b: &[u8]| {
            let s = String::from_utf8_lossy(b);
            let s = s.lines().filter(|l| !l.starts_with("warning:")).collect::<Vec<_>>().join("\n");
            serde_json::from_str(&s).unwrap_or(Value::Null)
        };
        (out.status.code().unwrap(), parse(&out.stdout), parse(&out.stderr))
    }

    fn with_account(self) -> Env {
        let (code, out, err) = self.json(&["accounts", "add", "work", "--api-key", "fc_test"]);
        assert_eq!(code, 0, "{err}");
        assert_eq!(out["status"], "added");
        self
    }
}

impl Drop for Env {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.dir);
    }
}

// =============================================================================================
// Tests
// =============================================================================================

#[test]
fn agent_readme_prints_json() {
    let mock = Mock::start(vec![]);
    let env = Env::new(&mock);
    let (code, out, err) = env.json(&["agent-readme"]);
    assert_eq!(code, 0, "{err}");
    assert_eq!(out["tool"], "firecrawl");
    assert_eq!(out["apiVersion"], "2.0.0");
    assert!(out["rules"].is_array());
    assert!(out["exitCodes"]["0"].is_string());
}

#[test]
fn agent_readme_prints_markdown() {
    let mock = Mock::start(vec![]);
    let env = Env::new(&mock);
    let out = env.run(&["agent-readme"]);
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("# firecrawl - agent operating manual"));
    assert!(stdout.contains("scrape"));
    assert!(stdout.contains("crawl"));
}

#[test]
fn no_account_error() {
    let mock = Mock::start(vec![]);
    let env = Env::new(&mock);
    let (code, _, err) = env.json(&["scrape", "https://example.com"]);
    assert_eq!(code, 7); // ErrorCode::NoAccount
    assert_eq!(err["code"], "no_account");
}

#[test]
fn accounts_add_list_test_remove() {
    let mock = Mock::start(vec![("GET", "team/credit-usage", 200, json!({"remaining_credits": 5000}))]);
    let env = Env::new(&mock);

    // 1. Add
    let (code, out, err) = env.json(&["accounts", "add", "personal", "--api-key", "fc_pers"]);
    assert_eq!(code, 0, "{err}");
    assert_eq!(out["status"], "added");
    assert_eq!(out["name"], "personal");
    assert_eq!(out["identity"], "credits: 5000");

    // 2. List
    let (code, list, err) = env.json(&["accounts", "list"]);
    assert_eq!(code, 0, "{err}");
    assert_eq!(list.as_array().unwrap().len(), 1);
    assert_eq!(list[0]["name"], "personal");

    // 3. Test
    let (code, test_res, err) = env.json(&["accounts", "test", "personal"]);
    assert_eq!(code, 0, "{err}");
    assert_eq!(test_res["status"], "valid");

    // 4. Remove
    let (code, rm_res, err) = env.json(&["accounts", "remove", "personal", "--yes"]);
    assert_eq!(code, 0, "{err}");
    assert_eq!(rm_res["status"], "removed");

    // 5. List is now empty
    let (_, list2, _) = env.json(&["accounts", "list"]);
    assert_eq!(list2.as_array().unwrap().len(), 0);
}

#[test]
fn scrape_command() {
    let mock = Mock::start(vec![
        ("GET", "team/credit-usage", 200, json!({"remaining_credits": 1000})),
        (
            "POST",
            "scrape",
            200,
            json!({
                "success": true,
                "data": {
                    "markdown": "# Example Domain",
                    "html": "<h1>Example Domain</h1>"
                }
            }),
        ),
    ]);
    let env = Env::new(&mock).with_account();

    let (code, out, err) = env.json(&[
        "scrape",
        "https://example.com",
        "-a",
        "work",
        "--formats",
        "markdown,html",
        "--only-clean-content",
        "--mobile",
    ]);
    assert_eq!(code, 0, "{err}");
    assert_eq!(out["success"], true);

    let req = mock.last("POST");
    assert_eq!(req.path, "scrape");
    let body = req.body.unwrap();
    assert_eq!(body["url"], "https://example.com");
    assert_eq!(body["formats"], json!(["markdown", "html"]));
    assert_eq!(body["onlyCleanContent"], true);
    assert_eq!(body["mobile"], true);
}

#[test]
fn crawl_lifecycle() {
    let job_id = "550e8400-e29b-41d4-a716-446655440000";
    let status_path = "crawl/550e8400-e29b-41d4-a716-446655440000";
    let mock = Mock::start(vec![
        ("GET", "team/credit-usage", 200, json!({"remaining_credits": 1000})),
        ("POST", "crawl", 200, json!({"id": job_id, "url": "https://api.firecrawl.dev/v2/crawl/job-123"})),
        ("GET", status_path, 200, json!({"status": "completed", "total": 12, "completed": 12, "creditsUsed": 12})),
        ("DELETE", status_path, 200, json!({"status": "cancelled"})),
    ]);
    let env = Env::new(&mock).with_account();

    // 1. Start crawl
    let (code, out, err) = env.json(&[
        "crawl",
        "start",
        "https://example.com",
        "-a",
        "work",
        "--limit",
        "50",
        "--max-depth",
        "2",
        "--formats",
        "markdown",
    ]);
    assert_eq!(code, 0, "{err}");
    assert_eq!(out["id"], job_id);

    let req = mock.last("POST");
    assert_eq!(req.path, "crawl");
    let body = req.body.unwrap();
    assert_eq!(body["limit"], 50);
    assert_eq!(body["maxDiscoveryDepth"], 2);
    assert_eq!(body["scrapeOptions"]["formats"], json!(["markdown"]));

    // 2. Status crawl
    let (code, out, err) = env.json(&["crawl", "status", job_id, "-a", "work"]);
    assert_eq!(code, 0, "{err}");
    assert_eq!(out["status"], "completed");

    // 3. Cancel crawl
    let (code, out, err) = env.json(&["crawl", "cancel", job_id, "-a", "work"]);
    assert_eq!(code, 0, "{err}");
    assert_eq!(out["status"], "cancelled");
}

#[test]
fn map_command() {
    let mock = Mock::start(vec![
        ("GET", "team/credit-usage", 200, json!({"remaining_credits": 1000})),
        (
            "POST",
            "map",
            200,
            json!({
                "success": true,
                "links": ["https://example.com/blog", "https://example.com/about"]
            }),
        ),
    ]);
    let env = Env::new(&mock).with_account();

    let (code, out, err) =
        env.json(&["map", "https://example.com", "-a", "work", "--search", "blog", "--limit", "100", "--ignore-cache"]);
    assert_eq!(code, 0, "{err}");
    assert_eq!(out["success"], true);

    let req = mock.last("POST");
    assert_eq!(req.path, "map");
    let body = req.body.unwrap();
    assert_eq!(body["url"], "https://example.com");
    assert_eq!(body["search"], "blog");
    assert_eq!(body["limit"], 100);
    assert_eq!(body["ignoreCache"], true);
}

#[test]
fn search_command() {
    let mock = Mock::start(vec![
        ("GET", "team/credit-usage", 200, json!({"remaining_credits": 1000})),
        (
            "POST",
            "search",
            200,
            json!({
                "success": true,
                "data": [
                    {"url": "https://rust-lang.org", "title": "Rust Programming Language"}
                ]
            }),
        ),
    ]);
    let env = Env::new(&mock).with_account();

    let (code, out, err) =
        env.json(&["search", "Rust programming", "-a", "work", "--limit", "5", "--formats", "markdown"]);
    assert_eq!(code, 0, "{err}");
    assert_eq!(out["success"], true);

    let req = mock.last("POST");
    assert_eq!(req.path, "search");
    let body = req.body.unwrap();
    assert_eq!(body["query"], "Rust programming");
    assert_eq!(body["limit"], 5);
    assert_eq!(body["scrapeOptions"]["formats"], json!(["markdown"]));
}

#[test]
fn direct_api_key_flag_and_env() {
    let mock = Mock::start(vec![("POST", "scrape", 200, json!({"success": true}))]);
    let env = Env::new(&mock);

    // 1. Direct flag
    let (code, out, err) = env.json(&["scrape", "https://example.com", "--api-key", "fc_flag"]);
    assert_eq!(code, 0, "{err}");
    assert_eq!(out["success"], true);
    let req = mock.last("POST");
    assert!(req.headers.iter().any(|(k, v)| k == "authorization" && v == "Bearer fc_flag"));

    // 2. Direct env var
    let args = vec!["scrape", "https://example.com", "--json"];
    let out = env.run_with_env(&args, "FIRECRAWL_API_KEY", "fc_from_env");
    assert!(out.status.success());
    let req2 = mock.last("POST");
    assert!(req2.headers.iter().any(|(k, v)| k == "authorization" && v == "Bearer fc_from_env"));
}

#[test]
fn error_code_401() {
    let mock = Mock::start(vec![("POST", "scrape", 401, json!({"error": "Invalid key"}))]);
    let env = Env::new(&mock);
    let (code, _, err) = env.json(&["scrape", "https://example.com", "--api-key", "bad_key"]);
    assert_eq!(code, 3);
    assert_eq!(err["code"], "auth_required");
}

#[test]
fn error_code_404() {
    let mock = Mock::start(vec![("POST", "scrape", 404, json!({"error": "Not found"}))]);
    let env = Env::new(&mock);
    let (code, _, err) = env.json(&["scrape", "https://example.com", "--api-key", "key"]);
    assert_eq!(code, 4);
    assert_eq!(err["code"], "not_found");
}

#[test]
fn error_code_429() {
    let mock = Mock::start(vec![("POST", "scrape", 429, json!({"error": "Too many requests"}))]);
    let env = Env::new(&mock);
    let (code, _, err) = env.json(&["scrape", "https://example.com", "--api-key", "key"]);
    assert_eq!(code, 5);
    assert_eq!(err["code"], "rate_limited");
}

#[test]
fn error_code_400() {
    let mock = Mock::start(vec![("POST", "scrape", 400, json!({"error": "Bad request"}))]);
    let env = Env::new(&mock);
    let (code, _, err) = env.json(&["scrape", "https://example.com", "--api-key", "key"]);
    assert_eq!(code, 6);
    assert_eq!(err["code"], "invalid_input");
}
