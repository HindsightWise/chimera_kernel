#!/usr/bin/env python3
"""
Test script to check if we can connect to Mnemosyne
and what the interface expects.
"""

import json
import sys
import os

# Try to import mnemosyne
try:
    # Add parent directory to path
    sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'Mnemosyne-Substrate', 'mnemosyne'))
    import mnemosyne as mn
    print("✓ Successfully imported mnemosyne module")
    
    # Try to create engine
    engine = mn.MnemosyneEngine()
    print("✓ Successfully created MnemosyneEngine")
    
    # Test with dummy embedding
    dummy_embedding = [0.1] * 384  # 384-dim vector (common for sentence-transformers)
    embedding_json = json.dumps(dummy_embedding)
    
    try:
        result = engine.query_semantic_memory(embedding_json, 5)
        print(f"✓ Query successful: {result[:100]}...")
    except Exception as e:
        print(f"✗ Query failed: {e}")
        
except ImportError as e:
    print(f"✗ Could not import mnemosyne: {e}")
    print("\nTrying to check if module is built...")
    
    # Check if the Python extension exists
    mnemosyne_path = "../Mnemosyne-Substrate/mnemosyne"
    if os.path.exists(mnemosyne_path):
        print(f"✓ Mnemosyne directory exists at: {mnemosyne_path}")
        
        # Check for compiled module
        import glob
        so_files = glob.glob(os.path.join(mnemosyne_path, "**/*.so"), recursive=True)
        dylib_files = glob.glob(os.path.join(mnemosyne_path, "**/*.dylib"), recursive=True)
        
        if so_files:
            print(f"✓ Found .so files: {so_files}")
        if dylib_files:
            print(f"✓ Found .dylib files: {dylib_files}")
            
        if not so_files and not dylib_files:
            print("✗ No compiled module found. May need to run: maturin develop --release")
            
except Exception as e:
    print(f"✗ Unexpected error: {e}")
