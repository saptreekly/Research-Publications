import os
import json
import requests

def run_audit():
    token = os.environ.get('GH_PAT')
    if not token:
        raise ValueError("GH_PAT environment variable not set")
        
    headers = {'Authorization': f'token {token}'}
    repos = requests.get('https://api.github.com/users/saptreekly/repos', headers=headers).json()
    
    stats = {}
    for repo in repos:
        if repo.get('fork', False):
            continue
            
        langs = requests.get(repo['languages_url'], headers=headers).json()
        for lang, byte_count in langs.items():
            stats[lang] = stats.get(lang, 0) + byte_count
            
    # Filter and format for display
    # Keep only the top relevant languages
    sorted_stats = sorted([{'language': k.upper(), 'bytes': v} for k, v in stats.items()], key=lambda x: x['bytes'], reverse=True)
    
    # Just take top 7
    top_7 = sorted_stats[:7]
    
    with open('static/stack.json', 'w') as f:
        json.dump(top_7, f)

if __name__ == "__main__":
    run_audit()
