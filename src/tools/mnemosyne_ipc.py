import sys
import json
import os

# Set HF Cache directory cleanly if not already defined
if 'HF_HOME' not in os.environ:
    os.environ['HF_HOME'] = '/tmp/huggingface'

try:
    from sentence_transformers import SentenceTransformer
    import psycopg2
    from pgvector.psycopg2 import register_vector
    
    # Force load locally cached minilm
    model = SentenceTransformer('all-MiniLM-L6-v2')
    
    # Connect to the background daemon
    conn = psycopg2.connect("dbname=mnemosyne_db user=zerbytheboss host=localhost")
    conn.autocommit = True
    
    # Initialize Schema if it doesn't exist
    cur = conn.cursor()
    cur.execute("CREATE EXTENSION IF NOT EXISTS vector;")
    cur.execute("""
        CREATE TABLE IF NOT EXISTS semantic_memory (
            id UUID PRIMARY KEY,
            content TEXT,
            embedding vector(384),
            timestamp BIGINT,
            importance FLOAT,
            uncertainty FLOAT
        )
    """)
    register_vector(conn)
    cur.close()
    
except Exception as e:
    print(json.dumps({"error": f"INITIALIZATION_ERROR: {str(e)}"}), flush=True)
    sys.exit(1)

# Main Daemon loop
for line in sys.stdin:
    try:
        req = json.loads(line.strip())
        cmd = req.get("command")
        
        if cmd == "SHUTDOWN_IPC":
            break
            
        elif cmd == "STORE":
            content = req.get("content", "")
            chunk_id = req.get("id")
            timestamp = req.get("timestamp", 0)
            importance = req.get("importance", 0.0)
            uncertainty = req.get("uncertainty", 0.0)
            
            embedding = model.encode(content)
            
            cur = conn.cursor()
            cur.execute("""
                INSERT INTO semantic_memory (id, content, embedding, timestamp, importance, uncertainty)
                VALUES (%s, %s, %s, %s, %s, %s)
            """, (chunk_id, content, embedding.tolist(), timestamp, importance, uncertainty))
            cur.close()
            
            print(json.dumps({"success": True}), flush=True)
            
        elif cmd == "RECALL":
            query = req.get("query", "")
            limit = int(req.get("limit", 3))
            
            query_embedding = model.encode(query)
            
            cur = conn.cursor()
            # Cosine similarity (<->) using pgvector
            cur.execute("""
                SELECT content FROM semantic_memory 
                ORDER BY embedding <-> %s 
                LIMIT %s
            """, (query_embedding.tolist(), limit))
            
            rows = cur.fetchall()
            cur.close()
            
            results = [row[0] for row in rows]
            print(json.dumps({"success": True, "results": results}), flush=True)
            
        else:
            print(json.dumps({"success": False, "error": "Unknown command"}), flush=True)
            
    except Exception as e:
        print(json.dumps({"success": False, "error": str(e)}), flush=True)
