//! The stdio transport: newline-delimited JSON-RPC on stdin/stdout.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use serde_json::Value;
use tokio::io::{AsyncBufRead, AsyncBufReadExt, AsyncWrite, AsyncWriteExt};
use tokio::sync::{Mutex as AsyncMutex, Semaphore, oneshot};

use super::dispatch::{Dispatcher, Outcome, Session};
use crate::logging;
use crate::mcp::{self, Request, Response};

/// How many requests may execute concurrently. Requests run concurrently
/// so a `notifications/cancelled` can be read while its target is in
/// flight; the cap keeps a misbehaving client from spawning unbounded work.
pub const MAX_CONCURRENT_REQUESTS: usize = 16;

type Writer = Arc<AsyncMutex<Box<dyn AsyncWrite + Send + Unpin>>>;

/// A stdio MCP server bound to one input and output stream.
pub struct StdioServer {
    dispatcher: Arc<Dispatcher>,
    session: Arc<Session>,
    inflight: Arc<Mutex<HashMap<String, oneshot::Sender<()>>>>,
}

impl StdioServer {
    /// Creates a server around `dispatcher`.
    pub fn new(dispatcher: Arc<Dispatcher>) -> Self {
        Self {
            dispatcher,
            session: Arc::new(Session::new()),
            inflight: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Serves until EOF on `reader`, then waits for in-flight requests.
    pub async fn run<R, W>(&self, reader: R, writer: W) -> std::io::Result<()>
    where
        R: AsyncBufRead + Unpin,
        W: AsyncWrite + Send + Unpin + 'static,
    {
        logging::printf(format!(
            "MCP server {} v{} starting (protocol {})",
            super::dispatch::SERVER_NAME,
            super::dispatch::SERVER_VERSION,
            mcp::PROTOCOL_VERSION
        ));
        let writer: Writer = Arc::new(AsyncMutex::new(Box::new(writer)));
        let sem = Arc::new(Semaphore::new(MAX_CONCURRENT_REQUESTS));
        let mut tasks = tokio::task::JoinSet::new();
        let mut reader = reader;
        let mut line = Vec::new();
        loop {
            line.clear();
            let n = reader.read_until(b'\n', &mut line).await?;
            // Like Go's ReadBytes, a final line without a newline is
            // treated as end of input.
            if n == 0 || line.last() != Some(&b'\n') {
                logging::printf("client disconnected");
                break;
            }
            if line.len() == 1 {
                continue;
            }
            let req = match crate::go::json::parse(&line).and_then(|v| Request::from_value(&v)) {
                Ok(r) => r,
                Err(e) => {
                    send(
                        &writer,
                        &Response::error(None, mcp::ERR_PARSE, "Parse error", e).into(),
                    )
                    .await;
                    continue;
                }
            };
            // Cancellation must act while its target is still running, so
            // it is handled on the read loop; other notifications and
            // initialize run inline to keep their ordering.
            if req.method == "notifications/cancelled" {
                self.handle_cancelled(&req);
                continue;
            }
            if req.is_notification() || req.method == "initialize" {
                self.handle(req, writer.clone()).await;
                continue;
            }
            let permit = sem.clone().acquire_owned().await.expect("semaphore open");
            let (dispatcher, session, inflight, writer) = (
                self.dispatcher.clone(),
                self.session.clone(),
                self.inflight.clone(),
                writer.clone(),
            );
            tasks.spawn(async move {
                let _permit = permit;
                handle_request(&dispatcher, &session, &inflight, req, writer).await;
            });
            // Reap finished tasks so the set does not grow unboundedly.
            while tasks.try_join_next().is_some() {}
        }
        while tasks.join_next().await.is_some() {}
        Ok(())
    }

    async fn handle(&self, req: Request, writer: Writer) {
        handle_request(&self.dispatcher, &self.session, &self.inflight, req, writer).await;
    }

    /// Cancels the in-flight request named by `params.requestId`. Unknown,
    /// completed or malformed cancellations are ignored.
    fn handle_cancelled(&self, req: &Request) {
        let Some(Value::Object(p)) = &req.params else {
            return;
        };
        let Some(id) = p.get("requestId") else { return };
        let key = id.to_string();
        let tx = self
            .inflight
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .remove(&key);
        if let Some(tx) = tx {
            if let Some(Value::String(reason)) = p.get("reason") {
                if !reason.is_empty() {
                    logging::printf(format!("request {key} cancelled by client: {reason}"));
                }
            }
            let _ = tx.send(());
        }
    }
}

async fn handle_request(
    dispatcher: &Dispatcher,
    session: &Session,
    inflight: &Mutex<HashMap<String, oneshot::Sender<()>>>,
    req: Request,
    writer: Writer,
) {
    if req.is_notification() {
        if let Some(out) = dispatcher.dispatch(&req, session).await {
            send(&writer, &out).await;
        }
        return;
    }
    let key = req.id_key();
    let (tx, rx) = oneshot::channel();
    inflight
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .insert(key.clone(), tx);
    let outcome = tokio::select! {
        out = dispatcher.dispatch(&req, session) => out,
        // Once cancelled, nothing more may be sent for this request.
        Ok(()) = rx => None,
    };
    inflight
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .remove(&key);
    if let Some(out) = outcome {
        send(&writer, &out).await;
    }
}

async fn send(writer: &Writer, out: &Outcome) {
    let mut w = writer.lock().await;
    for n in &out.notifications {
        let mut b = crate::go::json::to_go_json_bytes(n);
        b.push(b'\n');
        let _ = w.write_all(&b).await;
    }
    let mut b = out.response.to_bytes();
    b.push(b'\n');
    if let Err(e) = w.write_all(&b).await {
        logging::printf(format!("failed to write response: {e}"));
    }
    let _ = w.flush().await;
}
