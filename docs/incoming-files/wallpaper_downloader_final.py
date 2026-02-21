#!/usr/bin/env python3
"""
BBC/UNIX Retro-Futuristic Wallpaper Downloader - FINAL EDITION
Merges Claude's 6 categories + Bard's 2 categories = 8 total categories
Downloads 5000+ wallpapers focused on retro computing and gaming

Usage: python3 wallpaper_downloader_final.py [--category CATEGORY]
       python3 wallpaper_downloader_final.py --first-boot

Categories: retro, games, anime, dc, marvel, judge_dredd, star_wars, cyberpunk
"""

import os
import sys
import json
import time
import requests
import hashlib
import argparse
import subprocess
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor, as_completed
from typing import List, Dict, Set

# Configuration
WALLPAPER_DIR = Path.home() / "Pictures" / "RetroWallpapers"
MAX_WORKERS = 4  # Parallel downloads
TIMEOUT = 30  # Seconds per download
USER_AGENT = "MASH-Retro-Wallpaper-Downloader/3.0"

# Wallhaven API configuration
WALLHAVEN_BASE = "https://wallhaven.cc/api/v1/search"
WALLHAVEN_API_KEY = "YOUR_API_KEY_HERE"  # Replace with real key

# 8 Categories - Merged from both scripts (5000 total images)
CATEGORIES = {
    "retro": {
        "display": "Retro Computing",
        "queries": [
            "retro computer", "bbc micro", "unix workstation",
            "vintage tech", "old computer", "80s computer",
            "90s computer", "amiga", "commodore 64", "apple ii",
            "terminal", "command line", "text mode", "green screen"
        ],
        "count": 1000  # 1000 retro computing images
    },
    "games": {
        "display": "Video Games",
        "queries": [
            "retro video games", "arcade cabinets", "pixel art",
            "8-bit games", "16-bit games", "classic video games",
            "retro gaming", "videogame art", "game consoles",
            "nintendo", "sega", "atari", "playstation 1"
        ],
        "count": 1000  # 1000 game images
    },
    "anime": {
        "display": "Anime",
        "queries": [
            "retro anime", "cyberpunk anime", "80s anime",
            "90s anime", "anime computers", "anime technology"
        ],
        "count": 625  # 625 anime images
    },
    "dc": {
        "display": "DC Comics",
        "queries": [
            "dc comics", "batman", "superman", "justice league",
            "dc retro", "dc computers", "dc technology"
        ],
        "count": 625  # 625 DC images
    },
    "marvel": {
        "display": "Marvel Comics",
        "queries": [
            "marvel comics", "iron man", "spider man", "avengers",
            "marvel retro", "marvel computers", "marvel technology"
        ],
        "count": 625  # 625 Marvel images
    },
    "judge_dredd": {
        "display": "Judge Dredd/Lobo",
        "queries": [
            "judge dredd", "lobo", "2000 ad", "mega city one",
            "judge dredd computer", "lobo comic", "2000 ad retro"
        ],
        "count": 562  # 562 Judge Dredd images
    },
    "star_wars": {
        "display": "Star Wars",
        "queries": [
            "star wars retro", "star wars computers", "star wars technology",
            "droids", "star wars terminals", "star wars retro tech"
        ],
        "count": 562  # 562 Star Wars images
    },
    "cyberpunk": {
        "display": "Cyberpunk",
        "queries": [
            "cyberpunk", "cyberpunk computers", "cyberpunk terminals",
            "neon computers", "retro futurism", "cyberpunk technology",
            "hacker aesthetic", "terminal aesthetic"
        ],
        "count": 1000  # 1000 cyberpunk images
    }
}

# Total: 1000+1000+625+625+625+562+562+1000 = 5999 images
WALLHAVEN_PARAMS = {
    "categories": "111",  # General, Anime, People
    "purity": "100",      # SFW only
    "sorting": "relevance",
    "atleast": "1920x1080",  # Minimum resolution
    "apikey": WALLHAVEN_API_KEY
}

class RetroWallpaperDownloader:
    def __init__(self):
        self.output_dir = WALLPAPER_DIR
        self.session = requests.Session()
        self.session.headers.update({"User-Agent": USER_AGENT})
        self.downloaded_hashes: Set[str] = set()
        self.url_cache: Set[str] = set()
        self.success_count = 0
        self.fail_count = 0
        self.first_boot = False

    def set_first_boot_mode(self, enabled: bool):
        self.first_boot = enabled

    def log(self, message: str):
        if not self.first_boot:
            print(message)

    def create_directories(self):
        for category in CATEGORIES.keys():
            (self.output_dir / category).mkdir(parents=True, exist_ok=True)
        self.log(f"📁 Created directories in {self.output_dir}")

    def compute_file_hash(self, filepath: Path) -> str:
        sha256_hash = hashlib.sha256()
        with open(filepath, "rb") as f:
            for byte_block in iter(lambda: f.read(4096), b""):
                sha256_hash.update(byte_block)
        return sha256_hash.hexdigest()

    def load_existing_hashes(self):
        self.log("🔍 Scanning for existing files...")
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
                            self.log(f"  ✗ Error hashing {image_file}: {e}")
        
        if count > 0:
            self.log(f"✓ Loaded {count} existing files ({len(self.downloaded_hashes)} unique)")
        return count

    def download_image(self, url: str, filepath: Path) -> bool:
        if url in self.url_cache or filepath.exists():
            return False

        try:
            response = self.session.get(url, timeout=TIMEOUT, stream=True)
            response.raise_for_status()

            temp_path = filepath.with_suffix('.tmp')
            with open(temp_path, 'wb') as f:
                for chunk in response.iter_content(chunk_size=8192):
                    f.write(chunk)

            file_hash = self.compute_file_hash(temp_path)
            if file_hash in self.downloaded_hashes:
                temp_path.unlink()
                self.url_cache.add(url)
                return False

            temp_path.rename(filepath)
            self.downloaded_hashes.add(file_hash)
            self.url_cache.add(url)
            return True

        except Exception as e:
            self.log(f"✗ Error downloading {url}: {e}")
            if filepath.exists():
                filepath.unlink()
            temp_path = filepath.with_suffix('.tmp')
            if temp_path.exists():
                temp_path.unlink()
            return False

    def search_wallhaven(self, query: str, count: int) -> List[str]:
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

            except Exception as e:
                self.log(f"✗ Error searching Wallhaven for '{query}': {e}")
                break

        return images

    def download_category(self, category: str, queries: List[str], count: int):
        images_per_query = max(1, count // len(queries))
        all_images = []

        for query in queries:
            self.log(f"🔍 Searching for '{query}'...")
            images = self.search_wallhaven(query, images_per_query)
            all_images.extend(images)
            self.log(f"  ✓ Found {len(images)} images")

        downloaded = 0
        with ThreadPoolExecutor(max_workers=MAX_WORKERS) as executor:
            futures = []

            for i, url in enumerate(all_images[:count]):
                filename = self.output_dir / category / f"{category}_{downloaded:04d}.jpg"
                futures.append(executor.submit(
                    self.download_image, url, filename
                ))

            for future in as_completed(futures):
                if future.result():
                    downloaded += 1
                    self.success_count += 1
                    if downloaded % 10 == 0 and not self.first_boot:
                        print(f"  ✓ Downloaded {downloaded} images...")
                else:
                    self.fail_count += 1

        return downloaded

    def download_all(self, category_filter: str = "all", limit: int = 6000):
        self.create_directories()
        self.load_existing_hashes()

        start_time = time.time()
        total_target = min(limit, 6000)

        self.log(f"🚀 Starting download of {total_target} wallpapers...")
        self.log(f"📥 Category: {category_filter}")
        self.log(f"📁 Destination: {self.output_dir}")

        if category_filter == "all":
            # Download all 8 categories
            results = {}
            for category, data in CATEGORIES.items():
                results[category] = self.download_category(
                    category,
                    data["queries"],
                    data["count"]
                )
            
            total_downloaded = sum(results.values())
            
            # Print summary per category
            if not self.first_boot:
                print("\n📊 Category Summary:")
                for category, count in results.items():
                    display_name = CATEGORIES[category]["display"]
                    print(f"  {display_name}: {count} images")

        else:
            # Download specific category
            category_data = CATEGORIES.get(category_filter)
            if category_data:
                total_downloaded = self.download_category(
                    category_filter,
                    category_data["queries"],
                    min(limit, category_data["count"])
                )
            else:
                self.log("❌ Invalid category. Available: " + ", ".join(CATEGORIES.keys()))
                return False

        # Summary
        elapsed = time.time() - start_time
        self.log(f"\n📊 Download Complete!")
        self.log(f"✅ Success: {self.success_count}")
        self.log(f"❌ Failed: {self.fail_count}")
        self.log(f"⏱️  Time: {elapsed:.2f} seconds")
        self.log(f"📁 Location: {self.output_dir}")

        # Set as wallpaper directory
        self.setup_wallpapers()

        return True

    def setup_wallpapers(self):
        try:
            # For i3/feh
            config_file = Path.home() / ".config" / "i3" / "config"
            if config_file.exists():
                with open(config_file, 'a') as f:
                    f.write(f"\n# Auto-generated wallpaper setting\n")
                    f.write(f"exec_always feh --bg-scale --randomize {self.output_dir}/retro/* {self.output_dir}/games/* {self.output_dir}/anime/* {self.output_dir}/dc/* {self.output_dir}/marvel/* {self.output_dir}/judge_dredd/* {self.output_dir}/star_wars/* {self.output_dir}/cyberpunk/*\n")

            # For GNOME
            subprocess.run([
                "gsettings", "set",
                "org.gnome.desktop.background",
                "picture-uri",
                f"file://{self.output_dir}/retro/retro_0001.jpg"
            ], capture_output=True)

            self.log(f"🎨 Configured wallpaper directory")

        except Exception as e:
            self.log(f"⚠️  Could not auto-configure wallpapers: {e}")

def main():
    parser = argparse.ArgumentParser(
        description="BBC/UNIX Retro-Futuristic Wallpaper Downloader - 8 Categories"
    )
    parser.add_argument(
        "--category",
        choices=["all"] + list(CATEGORIES.keys()),
        default="all",
        help="Category to download (all, retro, games, anime, dc, marvel, judge_dredd, star_wars, cyberpunk)"
    )
    parser.add_argument(
        "--limit",
        type=int,
        default=6000,
        help="Maximum number of wallpapers to download (max 6000)"
    )
    parser.add_argument(
        "--first-boot",
        action="store_true",
        help="Run in first-boot mode (minimal output)"
    )

    args = parser.parse_args()

    downloader = RetroWallpaperDownloader()
    downloader.set_first_boot_mode(args.first_boot)

    if not args.first_boot:
        print("""
╔════════════════════════════════════════════════════════════╗
║  🖥️  BBC/UNIX RETRO WALLPAPER DOWNLOADER - 8 CATEGORIES  ║
║  • 1000 Retro Computing  • 1000 Video Games              ║
║  • 625 Anime             • 625 DC Comics                  ║
║  • 625 Marvel            • 562 Judge Dredd/Lobo          ║
║  • 562 Star Wars         • 1000 Cyberpunk                 ║
║  Total: 5999 retro-futuristic wallpapers!               ║
╚════════════════════════════════════════════════════════════╝
        """)

    success = downloader.download_all(
        category_filter=args.category,
        limit=args.limit
    )

    if success and not args.first_boot:
        print("\n🎉 Wallpaper download complete!")
        print(f"📁 Find your wallpapers in: {downloader.output_dir}")
        print("🎨 Use 'feh --bg-scale --randomize ~/Pictures/RetroWallpapers/*/*' to set random wallpaper")

    return 0

if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("\n⏹️  Download cancelled by user")
        sys.exit(1)
    except Exception as e:
        print(f"\n❌ Fatal error: {e}")
        sys.exit(1)
