#!/usr/bin/env python3
import requests
from bs4 import BeautifulSoup
import json
import xml.etree.ElementTree as ET
import time
from datetime import datetime

# Headers to mimic a real browser to pass basic bot checks
HEADERS = {
    'User-Agent': 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36',
    'Accept': 'text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7',
    'Accept-Language': 'en-US,en;q=0.9',
    'Accept-Encoding': 'gzip, deflate, br',
    'Connection': 'keep-alive',
    'Upgrade-Insecure-Requests': '1',
    'Sec-Fetch-Dest': 'document',
    'Sec-Fetch-Mode': 'navigate',
    'Sec-Fetch-Site': 'none',
    'Sec-Fetch-User': '?1',
}

def parse_cookie_string(cookie_string):
    """Parse a weirdly formatted cookie string dump from the user request into a simple dict mapping"""
    cookies = {}
    lines = cookie_string.strip().split('\n')
    for line in lines:
        parts = line.strip().split()
        if len(parts) >= 2:
            # Usually the first two parts of these browser extension cookie dumps are name and value
            # Let's handle the exceptions roughly.
            name = parts[0]
            val = parts[1]
            if name and val:
                cookies[name] = val
    return cookies

# User provided strings:
sci_x_cookies_str = """
__qca P1-de600fff-1614-446e-8b20-06ce5043bd0f
_ga GA1.1.826181757.1775584934
_ga_N4C1S5EPV1 GS2.1.s1775584933$o1$g1$t1775585186$j60$l0$h0
deviceType desktop_02379953b93cd223243db09f1dd4e5b9m
mc 69a364e1-249fb-d3c4b-1a686
PHPSESSID rgk4efq1nfnnlb09tc822582ev
PhysAccount sid69d546a4d6d0c%252F%252FWmVyYnl0aGVCb3Nz%252F%252Fbd55866af5428817feba14b3d7f28f1d
scx_analytics_consent granted
sp CgkIye4GEgMQ8BEKCQiiowYSAxDwEQoJCO7oBhIDEPARCggIiQ0SAxDwEQoJCNyJBhIDEPARCgkIuYoDEgMQ8BEKCQiCrQMSAxDwEQoJCOyyBhIDEPARCggIng0SAxDwEQoJCIX_AhIDEPARCgkIvbsGEgMQ8BEKCQiN3QESAxDwEQoJCOirBhIDEPARCgkIgLMGEgMQ8BEKCAjZZRIDEPARCgkIv4EDEgMQ8BEKCQjNswYSAxDwEQoJCL_MBhIDEPARCgkI5e8GEgMQ8BEKCQi87AYSAxDwEQoJCPu2BhIDEPARCgcICxIDEPARCgkIoO4GEgMQ8BEKCAiScRIDEPARCgkI0aYCEgMQ8BEKCQjerwYSAxDwEQ==
"""
sci_x_cookies = parse_cookie_string(sci_x_cookies_str)

aps_cookies_str = """
__cf_bm SQJcjOvqjXjuHG3ktkr8JMoiUrSC45BhrTWDz7bZ_nI-1775585240.5783517-1.0.1.1-UsxZo.PEO9qPbAzHa6iWuJyl_HMlOWAxSfQ5g.Dul4fAwAN7a5mzxOiLfi8f4ylPgaqBhOp6rmq3onrZhVBZTqvKudRPV31CznHM.E.xHD5uR03qaQYVTxZYFawfzi9d
_ga GA1.1.1404827159.1770748079
_ga_ZS5V2B2DR1 GS2.1.s1775585241$o2$g0$t1775585241$j60$l0$h1636792123
_gcl_au 1.1.27696568.1770748079
_hjSession_6555788 eyJpZCI6IjU0MTQ4MDYxLTE2YWYtNDlhNy1hYWU4LTI2Mjg1ZmEyYmI4YSIsImMiOjE3NzU1ODUyNDEwMjYsInMiOjEsInIiOjEsInNiIjowLCJzciI6MCwic2UiOjAsImZzIjowLCJzcCI6MH0=
_hjSessionUser_6555788 eyJpZCI6ImZkOGNhNmNlLTkzNzUtNTg2ZS05MWY4LWQwOWJlMGYzNTg1NiIsImNyZWF0ZWQiOjE3NzA3NDgwODA2MTQsImV4aXN0aW5nIjp0cnVlfQ==
apsjournals.session BAh7CEkiD3Nlc3Npb25faWQGOgZFVG86HVJhY2s6OlNlc3Npb246OlNlc3Npb25JZAY6D0BwdWJsaWNfaWRJIkU3YmI5MzY3ZTI3ZWM1ZTZiM2VjZTgzYmIzYmEwZGNjMzBiMzNlNTAyZWQzYmJlYWFmYjFhMTViNTM4MmMxZThmBjsARkkiB2lkBjsARkkiKTkxM2JjOWJiLWNmMmEtNDUzYy1hMjZjLWYyMTgxMGU2YTU4YgY7AEZJIgljc3JmBjsARkkiRTkzYmYyYmM3NmJiYTQ3YTBmM2IwYTgyMTk2MzAzNThlNzBhMzZmMjg2YWFjNWZhMGZkMWE1MTQ2NzVhY2Y5YjUGOwBG--e9bd0323b340d49a7803fa03c763b6a068397c01
cf_clearance 2eDfhH4zsbl26UwZMKK2E.GSSKgUjbrPYeCgrgf4DKY-1775585240-1.2.1.1-6YI__f2NCT1YcIAppe0cHfhQ4zUKtBVSUAD4tlZLzhfNt1vFhJNlEgPNk5WqfAk3dR2tJM76HOe_VMffmdFVork4Pr.INOwH_dgOAjd4Rl7k.wuQEutobvfmohj8qn6vLocSc_YMr79Ideb_cz3KOb5FQMKJEPBKtPFG_aRaYZyrff2.YO4bUVCdIhhUWwwQF5hyjHXnv7K1DbtKq1bGlk.LvpKaLUZhrRi.u4tvKnEDubcGo7RLsy5ARUn_SGuXuBlveaLzneV7CSekWo3oZNqaB.RIZ0oJ59sxm4WTQIll2t3lmkQlMGTIE46fjYcKyOKf5ZH7OHVQd2Fropertw
feathr_session_id 69d547d82f0e5e8d1ca4cb83
"""
aps_cookies = parse_cookie_string(aps_cookies_str)

cen_cookies_str = """
__adroll_fpc 2518c313e1c79031f22fafc3a91ebda5-1775585305983
__ar_v4 E3BVUWO6PFG7DM4B2CJBFW%3A20260407%3A2%7CAP4ZW7HTI5F4VHQILYYOQR%3A20260407%3A2%7CWYHAJR32OJDBHGIZKI4LFP%3A20260407%3A2
__eoi ID=ee0d4410c289856c:T=1775585304:RT=1775585304:S=AA-AfjafkJ6W21GqddMTOJO36guB
__pat -14400000
__pvi eyJpZCI6InYtbW5veG44eHU4bjRxaWE4MiIsImRvbWFpbiI6Ii5hY3Mub3JnIiwidGltZSI6MTc3NTU4NTMzNTA0Mn0%3D
_tbc %7Bkpex%7DCbfqZqFd2p17IlYTB5zjeB4PuToiLcCdW7Yc4_1Jmle1byFz7YgEKR4p-j9Vb5e4
"""
cen_cookies = parse_cookie_string(cen_cookies_str)

substack_cookies_str = """
__cf_bm QQtRyckKO2ifGw7gGjvISdeJKyElKUZBHSFmmMlgvnA-1775585455-1.0.1.1-muM1zwhEbcsMLce0G5qPWE8gYr7gNoHpJ7fxDcbT8nWArCqpqRR45DRwMH3T53o3y9h2IuLuaiiXKGMR0SdvBb4wtXy.I4v8LajbdAmWSGo
_dd_s rum=0&expire=1775586366193
_ga GA1.1.618034251.1770748575
ab_experiment_sampled %22false%22
ab_testing_id %22fd33f3df-945b-40b6-8824-1ff3a085f478%22
ajs_anonymous_id %2206ddacf3-bcc1-4941-9a76-d5efb62c4c19%22
cf_clearance 7EB5LlFpizc9Cy4m84vLp3Hz6z4p4kU4np2k6PSuA78-1775585456-1.2.1.1-aPfw80WdUW90ebKrQV2c.9b26lPHCT6fySP_W43aYdkvjYV6P6rf7aW_lP_wEqmVy1JiIs9tEEBwNqnqTEEhWyvVgWWPbB8cj4eNG3CXvSR4qZQz5rc17RzLhLgujJifd2x0VGG2UybkYAYExvDakrKipJdh8ULWZUgVdccf79Uwou7UeeV84kNVkVeHHieI.o6TCjleTCPRbypj2JzXWrlGImgYe2pG.caCzgrRCf3jXV7fl0_5fFQLL4nBU65bAVfoeXvn0KuCQW0ITJliCKnfTcAyFFapt_K4Jjh0iC.DeFbdMS0_liLv2BBJKxthuGWZO9nJs4Cxmj6Uevkygg
"""
substack_cookies = parse_cookie_string(substack_cookies_str)

azom_cookies_str = """
__eoi ID=b0c5b549746afc12:T=1775585602:RT=1775585602:S=AA-AfjZCLw4U7_FaDKduQnZEjJcv
__qca P1-8892cb5a-ffbf-4d23-8177-c3f58b93f0a6
OptanonConsent isGpcEnabled=0&datestamp=Tue+Apr+07+2026+11%3A13%3A28+GMT-0700+(Pacific+Daylight+Time)&version=202601.1.0&browserGpcFlag=0&isIABGlobal=false&hosts=&consentId=d96f79cd-0a9e-4831-adb5-8b56cac9eca6&interactionCount=1&isAnonUser=1&landingPath=NotLandingPage&GPPCookiesCount=1&gppSid=7&groups=C0001%3A1%2CC0004%3A1%2CC0002%3A1%2CC0003%3A1&intType=1&crTime=1775585608460
"""
azom_cookies = parse_cookie_string(azom_cookies_str)

# Standard targets
TARGETS = [
    {"name": "Interesting Engineering", "url": "https://interestingengineering.com", "cookies": {}},
    {"name": "ScienceDaily Materials", "url": "https://www.sciencedaily.com/news/matter_energy/materials_science/", "cookies": {}},
    {"name": "APS Journals", "url": "https://journals.aps.org/prapplied/recent", "cookies": aps_cookies},
    {"name": "AZoM", "url": "https://www.azom.com/materials-news-index.aspx", "cookies": azom_cookies},
    {"name": "Phys.org Chemistry", "url": "https://phys.org/chemistry-news/", "cookies": sci_x_cookies},
    {"name": "C&EN", "url": "https://cen.acs.org/index.html", "cookies": cen_cookies},
    {"name": "Substack", "url": "https://substack.com/home", "cookies": substack_cookies},
    {"name": "ChemistryWorld", "url": "https://www.chemistryworld.com", "cookies": {}, "auth": ('zerbytheboss@gmail.com', 'E3EUWwvdUu#HE#g')}
]

ARXIV_CATEGORIES = [
    "cs.IR", "cs.FL", "cs.DL", "cs.DB", "cs.CR", "cs.CC", "cs.HC", "cs.ET", "math"
]

all_research = {
    "timestamp": datetime.now().isoformat(),
    "sources": []
}

# 1. Fetch web targets
for target in TARGETS:
    print(f"Fetch {target['name']}...")
    try:
        if 'auth' in target:
            resp = requests.get(target['url'], headers=HEADERS, cookies=target.get('cookies', {}), auth=target['auth'], timeout=10)
        else:
            resp = requests.get(target['url'], headers=HEADERS, cookies=target.get('cookies', {}), timeout=10)
        
        soup = BeautifulSoup(resp.text, 'html.parser')
        # Extract headlines/links broadly (h1, h2, h3 or elements with 'title' / 'link' classes)
        articles = []
        for tag in soup.find_all(['h2', 'h3']):
            text = tag.get_text(strip=True)
            if len(text) > 20: 
                link = tag.find('a')
                href = link['href'] if link and link.has_attr('href') else None
                articles.append({"headline": text, "url": href})
        
        # If headers fail, extract paragraphs up to a limit
        if len(articles) < 5:
            for p in soup.find_all('p'):
                text = p.get_text(strip=True)
                if len(text) > 50:
                    articles.append({"headline": text[:100] + "...", "url": None})
                    if len(articles) > 15:
                        break

        all_research["sources"].append({
            "name": target['name'],
            "status": "success",
            "articles": articles[:15]
        })
    except Exception as e:
        print(f"Error fetching {target['name']}: {str(e)}")
        all_research["sources"].append({
            "name": target['name'],
            "status": f"error: {str(e)}"
        })
    time.sleep(1) # just sleep

# 2. Fetch arXiv targets (using similar export api logic)
for cat in ARXIV_CATEGORIES:
    print(f"Fetch arXiv {cat}...")
    try:
        base_url = "https://export.arxiv.org/api/query"
        params = {"search_query": f"cat:{cat}", "start": 0, "max_results": 5, "sortBy": "submittedDate", "sortOrder": "descending"}
        resp = requests.get(base_url, params=params, timeout=10)
        root = ET.fromstring(resp.content)
        namespace = {'atom': 'http://www.w3.org/2005/Atom'}
        papers = []
        for entry in root.findall('atom:entry', namespace):
            title = entry.find('atom:title', namespace).text.strip().replace('\n', ' ')
            summary = entry.find('atom:summary', namespace).text.strip().replace('\n', ' ')
            papers.append({"title": title, "abstract": summary[:200] + "..."})
        
        all_research["sources"].append({
            "name": f"arXiv {cat}",
            "status": "success",
            "papers": papers
        })
    except Exception as e:
        print(f"Error fetching arXiv {cat}: {str(e)}")

with open("xenoactualized_research_run.json", "w") as f:
    json.dump(all_research, f, indent=2)

print("\n✅ XENOACTUALIZED RESEARCH ABSORPTION COMPLETE.")
