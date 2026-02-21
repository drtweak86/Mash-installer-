#!/usr/bin/env python3
"""
Wallpaper Downloader
Downloads ~1000 wallpapers across multiple categories from various sources
"""

import os
import json
import time
import requests
import hashlib
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor, as_completed
from typing import List, Dict, Set
import argparse

# Configuration
OUTPUT_DIR = Path.home() / "wallpapers"
IMAGES_PER_CATEGORY = {
    "anime": 167,
    "dc_comics": 167,
    "marvel": 167,
    "judge_dredd_lobo": 167,
    "star_wars": 167,
    "cyberpunk": 165
}

# Wallhaven API configuration
WALLHAVEN_BASE = "https://wallhaven.cc/api/v1/search"
WALLHAVEN_PARAMS = {
    "categories": "111",  # General, Anime, People
    "purity": "100",      # SFW only
    "sorting": "relevance",
    "atleast": "1920x1080"  # Minimum resolution
}

class WallpaperDownloader:
    def __init__(self, output_dir: Path):
        self.output_dir = output_dir
        self.session = requests.Session()
        self.session.headers.update({
            "User-Agent": "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36"
        })
        self.downloaded_hashes: Set[str] = set()
        self.url_cache: Set[str] = set()  # Track URLs to avoid re-downloading
        
    def create_directories(self):
        """Create output directory structure"""
        for category in IMAGES_PER_CATEGORY.keys():
            (self.output_dir / category).mkdir(parents=True, exist_ok=True)
        print(f"✓ Created directories in {self.output_dir}")
    
    def compute_file_hash(self, filepath: Path) -> str:
        """Compute SHA256 hash of a file"""
        sha256_hash = hashlib.sha256()
        with open(filepath, "rb") as f:
            for byte_block in iter(lambda: f.read(4096), b""):
                sha256_hash.update(byte_block)
        return sha256_hash.hexdigest()
    
    def load_existing_hashes(self):
        """Load hashes of existing files to prevent duplicates"""
        print("🔍 Scanning for existing files...")
        count = 0
        for category_dir in self.output_dir.iterdir():
            if category_dir.is_dir():
                for image_file in category_dir.glob("*"):
                    if image_file.is_file():
                        try:
                            file_hash = self.compute_file_hash(image_file)
                            self.downloaded_hashes.add(file_hash)
                            count += 1
                        except Exception as e:
                            print(f"  ✗ Error hashing {image_file}: {e}")
        
        if count > 0:
            print(f"✓ Loaded {count} existing files ({len(self.downloaded_hashes)} unique)")
        return count
    
    def download_image(self, url: str, filepath: Path) -> bool:
        """Download a single image with duplicate detection"""
        # Check if URL already processed
        if url in self.url_cache:
            return False
        
        # Check if file already exists
        if filepath.exists():
            return False
        
        try:
            response = self.session.get(url, timeout=30, stream=True)
            response.raise_for_status()
            
            # Download to temporary file first
            temp_path = filepath.with_suffix('.tmp')
            
            with open(temp_path, 'wb') as f:
                for chunk in response.iter_content(chunk_size=8192):
                    f.write(chunk)
            
            # Compute hash to check for duplicates
            file_hash = self.compute_file_hash(temp_path)
            
            if file_hash in self.downloaded_hashes:
                # Duplicate found, delete temp file
                temp_path.unlink()
                self.url_cache.add(url)
                return False
            
            # Not a duplicate, move to final location
            temp_path.rename(filepath)
            self.downloaded_hashes.add(file_hash)
            self.url_cache.add(url)
            
            return True
            
        except Exception as e:
            print(f"✗ Error downloading {url}: {e}")
            if filepath.exists():
                filepath.unlink()
            # Clean up temp file if it exists
            temp_path = filepath.with_suffix('.tmp')
            if temp_path.exists():
                temp_path.unlink()
            return False
    
    def search_wallhaven(self, query: str, count: int) -> List[str]:
        """Search Wallhaven for wallpapers"""
        images = []
        page = 1
        
        while len(images) < count and page <= 50:
            try:
                params = WALLHAVEN_PARAMS.copy()
                params.update({"q": query, "page": page})
                
                response = self.session.get(WALLHAVEN_BASE, params=params, timeout=10)
                response.raise_for_status()
                
                data = response.json()
                
                if not data.get("data"):
                    break
                
                for item in data["data"]:
                    if len(images) >= count:
                        break
                    images.append(item["path"])
                
                page += 1
                time.sleep(1)  # Rate limiting
                
            except Exception as e:
                print(f"✗ Error searching Wallhaven: {e}")
                break
        
        return images
    
    def download_reddit_wallpapers(self, subreddit: str, category: str, count: int):
        """Download wallpapers from Reddit"""
        images_downloaded = 0
        after = None
        
        while images_downloaded < count:
            try:
                url = f"https://www.reddit.com/r/{subreddit}/top.json"
                params = {"limit": 100, "t": "all"}
                if after:
                    params["after"] = after
                
                response = self.session.get(url, params=params, timeout=10)
                response.raise_for_status()
                
                data = response.json()
                posts = data["data"]["children"]
                
                if not posts:
                    break
                
                for post in posts:
                    if images_downloaded >= count:
                        break
                    
                    post_data = post["data"]
                    url = post_data.get("url", "")
                    
                    # Check if it's an image
                    if any(url.endswith(ext) for ext in [".jpg", ".png", ".jpeg"]):
                        filename = self.output_dir / category / f"reddit_{os.path.basename(url)}"
                        if self.download_image(url, filename):
                            images_downloaded += 1
                            print(f"  [{images_downloaded}/{count}] Downloaded from r/{subreddit}")
                
                after = data["data"]["after"]
                if not after:
                    break
                
                time.sleep(2)  # Reddit rate limiting
                
            except Exception as e:
                print(f"✗ Error downloading from r/{subreddit}: {e}")
                break
    
    def download_category(self, category: str, queries: List[str], count: int):
        """Download wallpapers for a category"""
        print(f"\n=== Category: {category.replace('_', ' ').title()} ===")
        
        images_per_query = count // len(queries)
        total_downloaded = 0
        
        for query in queries:
            print(f"\n  Searching: {query}")
            urls = self.search_wallhaven(query, images_per_query)
            
            print(f"  Found {len(urls)} images")
            
            # Download with threading
            with ThreadPoolExecutor(max_workers=5) as executor:
                futures = []
                for i, url in enumerate(urls):
                    filename = self.output_dir / category / f"{query.replace(' ', '_')}_{i}_{os.path.basename(url)}"
                    futures.append(executor.submit(self.download_image, url, filename))
                
                downloaded = 0
                for future in as_completed(futures):
                    if future.result():
                        downloaded += 1
                        total_downloaded += 1
                        print(f"  [{downloaded}/{len(urls)}] Downloaded", end="\r")
            
            print(f"\n  ✓ Downloaded {downloaded} images for '{query}'")
        
        return total_downloaded
    
    def run(self):
        """Main download process"""
        print("🖼️  Wallpaper Downloader Starting...\n")
        
        self.create_directories()
        self.load_existing_hashes()  # Load existing files to prevent duplicates
        
        # Define search queries for each category
        categories = {
            "anime": [
                "anime -ghibli landscape",
                "anime scenery",
                "anime city",
                "anime sunset"
            ],
            "dc_comics": [
                "batman",
                "joker",
                "harley quinn",
                "punisher"
            ],
            "marvel": [
                "deadpool",
                "marvel comics",
                "spider-man",
                "iron man"
            ],
            "judge_dredd_lobo": [
                "judge dredd",
                "lobo dc",
                "2000ad",
                "comic book art"
            ],
            "star_wars": [
                "star wars",
                "darth vader",
                "millennium falcon",
                "jedi"
            ],
            "cyberpunk": [
                "cyberpunk city",
                "matrix",
                "neon city",
                "cyber future"
            ]
        }
        
        total_overall = 0
        
        for category, queries in categories.items():
            target_count = IMAGES_PER_CATEGORY[category]
            downloaded = self.download_category(category, queries, target_count)
            total_overall += downloaded
        
        # Print summary
        print("\n" + "="*50)
        print("📊 Download Summary")
        print("="*50)
        
        for category in IMAGES_PER_CATEGORY.keys():
            count = len(list((self.output_dir / category).glob("*")))
            print(f"{category.replace('_', ' ').title():20s}: {count:4d} images")
        
        print(f"\n{'Total':20s}: {total_overall:4d} images")
        print(f"\n✓ Wallpapers saved to: {self.output_dir}")

def main():
    parser = argparse.ArgumentParser(description="Download wallpapers from multiple sources")
    parser.add_argument("-o", "--output", type=str, default=str(OUTPUT_DIR),
                       help="Output directory for wallpapers")
    args = parser.parse_args()
    
    downloader = WallpaperDownloader(Path(args.output))
    downloader.run()

if __name__ == "__main__":
    main()
