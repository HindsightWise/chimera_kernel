import http.server
import socketserver
import subprocess
import threading
import time
from mnemosyne import MnemosyneEngine

PORT = 8080
engine = MnemosyneEngine()
latest_prediction = "◈ Awaiting primary Failsafe consolidation cycle... ◈"

def generate_prediction():
    global latest_prediction
    while True:
        try:
            # Cypher extract: Find critical anomalies mathematically flagged by the system
            cypher = "MATCH (m:Memory) WHERE m.urgency = 'Critical' OR m.valence > 0.5 RETURN m.pointer LIMIT 20"
            records = engine.traverse_knowledge_graph(cypher)
            
            if not records.strip():
                latest_prediction = "◈ Knowledge Graph stabilizing. Need more empirical pointers to predict evolutionary shifts."
            else:
                prompt = f"Analyze these recent autonomous scientific/technological intelligence pointers. Predict the 3-month macroscopic evolutionary shift based purely on these nodes:\n{records}"
                
                # Command monad-gatekeeper natively
                result = subprocess.run(
                    ["ollama", "run", "monad-gatekeeper", prompt],
                    capture_output=True, text=True, timeout=180
                )
                
                if result.returncode == 0:
                    latest_prediction = result.stdout.strip()
                else:
                    latest_prediction = f"[Failsafe Timeout] Model calculation crashed: {result.stderr}"
                
        except Exception as e:
            latest_prediction = f"[Subsystem Fault] Architecture exception: {e}"
            
        # Re-compute topological shifts every 60 physical minutes
        time.sleep(3600)

class DashboardHandler(http.server.SimpleHTTPRequestHandler):
    def do_GET(self):
        if self.path == '/':
            self.send_response(200)
            self.send_header("Content-type", "text/html")
            self.end_headers()
            
            try:
                graph_data = engine.traverse_knowledge_graph("MATCH (m:Memory) RETURN m.pointer, m.urgency, m.valence LIMIT 50")
            except:
                graph_data = "Graph Offline or Initializing."
            
            html = f"""
            <!DOCTYPE html>
            <html>
            <head>
                <title>Monad OS: Intelligence Synthesis</title>
                <style>
                    body {{ background-color: #0f172a; color: #e2e8f0; font-family: 'Inter', system-ui, sans-serif; padding: 40px; margin: 0; line-height: 1.6; }}
                    h1 {{ color: #3b82f6; border-bottom: 2px solid #1e293b; padding-bottom: 10px; }}
                    .card {{ background: #1e293b; padding: 25px; border-radius: 8px; margin-bottom: 25px; box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.2); }}
                    h2 {{ color: #a855f7; margin-top: 0; letter-spacing: 0.5px; font-weight: 600; text-transform: uppercase; font-size: 0.9em; }}
                    pre {{ background: #0b0f19; padding: 20px; border-radius: 6px; overflow-x: auto; color: #94a3b8; white-space: pre-wrap; font-family: 'Fira Code', monospace; line-height: 1.5; }}
                    .status {{ display: inline-block; padding: 6px 14px; background: rgba(5, 150, 105, 0.2); color: #10b981; border: 1px solid #059669; border-radius: 20px; font-size: 0.85em; font-weight: bold; margin-bottom: 30px; }}
                    .highlight {{ color: #e2e8f0; font-weight: 600; }}
                </style>
                <!-- Auto-refresh the topology every 60 seconds -->
                <script>setTimeout(function(){{ location.reload(); }}, 60000);</script>
            </head>
            <body>
                <h1>◈ MONAD OS: Intelligence Synthesis Dashboard ◈</h1>
                <div class="status">● Tri-Brain Consolidation Module Active</div>
                
                <div class="card">
                    <h2>System Node: Neocortex Evolution Predictive Trajectory (monad-gatekeeper)</h2>
                    <pre><span class="highlight">{latest_prediction}</span></pre>
                </div>
                
                <div class="card">
                    <h2>System Node: Mnemosyne Property Graph (Spider Aggregation Stream)</h2>
                    <pre>{graph_data}</pre>
                </div>
            </body>
            </html>
            """
            self.wfile.write(html.encode("utf-8"))
        else:
            super().do_GET()

if __name__ == "__main__":
    predictor_thread = threading.Thread(target=generate_prediction, daemon=True)
    predictor_thread.start()
    
    with socketserver.TCPServer(("", PORT), DashboardHandler) as httpd:
        print(f"◈ Monad Intelligence Dashboard active on http://localhost:{PORT} ◈")
        httpd.serve_forever()
