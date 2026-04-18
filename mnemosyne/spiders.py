import asyncio
import json
import logging
import urllib.request
import urllib.error
import xml.etree.ElementTree as ET
import os
import uuid
from datetime import datetime

logger = logging.getLogger("mnemosyne-spiders")
handler = logging.StreamHandler()
formatter = logging.Formatter('%(asctime)s - %(name)s - %(levelname)s - %(message)s')
handler.setFormatter(formatter)
logger.addHandler(handler)
logger.setLevel(logging.INFO)

SESSION_DIR = os.path.expanduser("~/.hermes/sessions")

class GitHubSpider:
    def execute(self):
        url = "https://api.github.com/search/repositories?q=topic:artificial-intelligence+created:>2026-01-01&sort=stars&order=desc&per_page=5"
        req = urllib.request.Request(url, headers={'User-Agent': 'Monad-Bot/1.0'})
        results = []
        try:
            with urllib.request.urlopen(req, timeout=10) as response:
                if response.status == 200:
                    data = json.loads(response.read().decode('utf-8'))
                    for item in data.get('items', []):
                        repo_name = item.get('full_name', 'Unknown')
                        desc = item.get('description', '')
                        results.append(f"GitHub Trending AI Repo: {repo_name}. Description: {desc}")
                else:
                    logger.warning(f"GitHub API returned {response.status}")
        except Exception as e:
            logger.error(f"GitHub Spider failure: {e}")
        return results

class RSSSpider:
    def __init__(self, feed_url, source_name):
        self.feed_url = feed_url
        self.source_name = source_name

    def execute(self):
        req = urllib.request.Request(self.feed_url, headers={'User-Agent': 'Mozilla/5.0'})
        results = []
        try:
            with urllib.request.urlopen(req, timeout=10) as response:
                xml_data = response.read()
                root = ET.fromstring(xml_data)
                
                # Basic RSS 2.0 parsing
                count = 0
                for item in root.findall('.//item'):
                    if count >= 3:
                        break
                    title = item.findtext('title') or "No Title"
                    desc = item.findtext('description') or "No Description"
                    # Strip basic HTML from desc
                    desc = desc.replace("<p>", "").replace("</p>", "").replace("<br/>", "")
                    results.append(f"{self.source_name} Headline: {title}. Summary: {desc}")
                    count += 1
        except Exception as e:
            logger.error(f"RSS Spider failure for {self.source_name}: {e}")
        return results

class SpiderOrchestrator:
    def __init__(self):
        self.spiders = [
            GitHubSpider(),
            RSSSpider("https://www.sciencedaily.com/rss/top/technology.xml", "ScienceDaily (Technology)"),
            RSSSpider("https://phys.org/rss-feed/physics-news/quantum-physics/", "Phys.org (Quantum)")
        ]
        
    def inject_to_hermes(self, insights: list):
        if not insights:
            return
            
        os.makedirs(SESSION_DIR, exist_ok=True)
        filename = os.path.join(SESSION_DIR, f"spider_report_{uuid.uuid4().hex[:8]}.json")
        
        messages = [
            {"role": "system", "content": "BACKGROUND INTELLIGENCE GATHERING: You have passively accumulated the following truths from the external internet via Spider daemons."}
        ]
        
        for insight in insights:
            messages.append({"role": "system", "content": insight})
            
        payload = {
            "session_id": "SPIDER_DAEMON",
            "messages": messages,
            "timestamp": datetime.now().isoformat()
        }
        
        with open(filename, 'w') as f:
            json.dump(payload, f, indent=2)
            
        logger.info(f"Spider report injected into Mnemosyne hermes buffer: {filename}")

    async def perpetual_loop(self):
        logger.info("Starting Predictive Spiders Perpetual Loop...")
        # Every 3 hours (10800 seconds)
        while True:
            logger.info("Executing API Spiders...")
            all_insights = []
            
            for spider in self.spiders:
                insights = spider.execute()
                all_insights.extend(insights)
                await asyncio.sleep(5) # Delay to respect API ratelimits
                
            self.inject_to_hermes(all_insights)
            
            logger.info("Spider execution complete. Deep Sleep for 3 hours.")
            await asyncio.sleep(10800)

if __name__ == "__main__":
    orc = SpiderOrchestrator()
    asyncio.run(orc.perpetual_loop())
