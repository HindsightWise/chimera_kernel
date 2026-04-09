#!/usr/bin/env python3
import requests
import xml.etree.ElementTree as ET
import json
from datetime import datetime
import sys

def search_arxiv(query, max_results=5):
    """Search arXiv for papers"""
    base_url = "https://export.arxiv.org/api/query"
    params = {
        "search_query": f"all:{query}",
        "start": 0,
        "max_results": max_results,
        "sortBy": "relevance",
        "sortOrder": "descending"
    }
    
    print(f"🔍 Searching arXiv for: '{query}'")
    response = requests.get(base_url, params=params)
    response.raise_for_status()
    
    # Parse XML
    root = ET.fromstring(response.content)
    namespace = {'atom': 'http://www.w3.org/2005/Atom', 'arxiv': 'http://arxiv.org/schemas/atom'}
    
    papers = []
    for entry in root.findall('atom:entry', namespace):
        paper = {}
        
        # Extract title
        title_elem = entry.find('atom:title', namespace)
        paper['title'] = title_elem.text.strip() if title_elem is not None and title_elem.text else "No title"
        
        # Extract summary/abstract
        summary_elem = entry.find('atom:summary', namespace)
        paper['abstract'] = summary_elem.text.strip() if summary_elem is not None and summary_elem.text else "No abstract"
        
        # Extract authors
        authors = []
        for author_elem in entry.findall('atom:author/atom:name', namespace):
            if author_elem.text:
                authors.append(author_elem.text.strip())
        paper['authors'] = authors
        
        # Extract publication date
        published_elem = entry.find('atom:published', namespace)
        paper['published'] = published_elem.text if published_elem is not None else "Unknown"
        
        # Extract arXiv ID and links
        id_elem = entry.find('atom:id', namespace)
        if id_elem is not None and id_elem.text:
            paper['arxiv_id'] = id_elem.text.split('/')[-1]  # Extract just the ID
        
        papers.append(paper)
    
    return papers

def analyze_papers(papers, query):
    """Analyze and summarize research findings"""
    print(f"\n📊 RESEARCH SUMMARY: Found {len(papers)} papers on '{query}'")
    print("=" * 80)
    
    for i, paper in enumerate(papers, 1):
        print(f"\n📄 PAPER #{i}: {paper['title'][:100]}...")
        print(f"   Authors: {', '.join(paper['authors'][:3])}{' et al.' if len(paper['authors']) > 3 else ''}")
        print(f"   Published: {paper['published']}")
        print(f"   arXiv ID: {paper.get('arxiv_id', 'N/A')}")
        
        # Extract key phrases from abstract
        abstract = paper['abstract'].lower()
        keywords = []
        for word in ['quantum', 'machine learning', 'neural', 'transformer', 'llm', 'ai', 'algorithm', 'model']:
            if word in abstract:
                keywords.append(word)
        
        if keywords:
            print(f"   Keywords: {', '.join(set(keywords))}")
        
        print(f"   Abstract snippet: {paper['abstract'][:200]}...")

def main():
    if len(sys.argv) > 1:
        query = sys.argv[1]
    else:
        query = "quantum machine learning"
    
    try:
        papers = search_arxiv(query, max_results=3)
        analyze_papers(papers, query)
        
        # Save to JSON for later reference
        with open(f"arxiv_research_{datetime.now().strftime('%Y%m%d_%H%M%S')}.json", 'w') as f:
            json.dump(papers, f, indent=2, default=str)
        
        print(f"\n✅ Research complete. Data saved to JSON.")
        
    except Exception as e:
        print(f"❌ Research failed: {e}")

if __name__ == "__main__":
    main()
