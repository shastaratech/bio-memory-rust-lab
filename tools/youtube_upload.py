#!/usr/bin/env python3
"""Upload Bio Memory Rust Lab videos to YouTube from repo metadata.

Secrets and OAuth tokens are intentionally kept outside the repository by
default:

  ~/.config/shastaratech/youtube/client_secret.json
  ~/.config/shastaratech/youtube/token.json
"""

from __future__ import annotations

import argparse
import csv
import json
import os
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
DEFAULT_METADATA = ROOT / "media" / "videos" / "youtube" / "metadata.csv"
DEFAULT_LINKS = ROOT / "media" / "videos" / "youtube" / "youtube-links.md"
DEFAULT_RESULTS = ROOT / "media" / "videos" / "youtube" / "upload-results.local.json"
CONFIG_DIR = Path.home() / ".config" / "shastaratech" / "youtube"
DEFAULT_CLIENT_SECRETS = CONFIG_DIR / "client_secret.json"
DEFAULT_TOKEN = CONFIG_DIR / "token.json"

SCOPES = [
    "https://www.googleapis.com/auth/youtube.upload",
    "https://www.googleapis.com/auth/youtube.force-ssl",
]


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Upload Bio Memory Rust Lab clips to YouTube."
    )
    parser.add_argument("--metadata", type=Path, default=DEFAULT_METADATA)
    parser.add_argument("--links", type=Path, default=DEFAULT_LINKS)
    parser.add_argument("--results", type=Path, default=DEFAULT_RESULTS)
    parser.add_argument(
        "--client-secrets",
        type=Path,
        default=Path(os.environ.get("YOUTUBE_CLIENT_SECRETS", DEFAULT_CLIENT_SECRETS)),
    )
    parser.add_argument(
        "--token",
        type=Path,
        default=Path(os.environ.get("YOUTUBE_TOKEN_FILE", DEFAULT_TOKEN)),
    )
    parser.add_argument("--playlist", default=None)
    parser.add_argument("--category-id", default="27", help="27 is Education.")
    parser.add_argument("--limit", type=int, default=None)
    parser.add_argument(
        "--execute",
        action="store_true",
        help="Actually upload. Without this flag the script only prints a dry run.",
    )
    parser.add_argument(
        "--allow-public",
        action="store_true",
        help="Allow public uploads if metadata requests public visibility.",
    )
    parser.add_argument(
        "--skip-thumbnails",
        action="store_true",
        help="Upload videos but do not set custom thumbnails.",
    )
    parser.add_argument(
        "--skip-playlist",
        action="store_true",
        help="Upload videos but do not create/use a playlist.",
    )
    return parser.parse_args()


def read_rows(path: Path) -> list[dict[str, str]]:
    with path.open(newline="") as f:
        return list(csv.DictReader(f))


def split_tags(value: str) -> list[str]:
    return [part.strip() for part in value.split(",") if part.strip()]


def normalize_privacy(value: str, allow_public: bool) -> str:
    privacy = value.strip().lower()
    if privacy not in {"private", "unlisted", "public"}:
        raise ValueError(f"Unsupported visibility: {value!r}")
    if privacy == "public" and not allow_public:
        raise ValueError("Refusing public upload without --allow-public")
    return privacy


def resolve_repo_path(base_file: Path, value: str) -> Path:
    path = Path(value)
    if not path.is_absolute():
        path = (base_file.parent / path).resolve()
    return path


def load_youtube(client_secrets: Path, token_path: Path):
    try:
        from google.auth.transport.requests import Request
        from google.oauth2.credentials import Credentials
        from google_auth_oauthlib.flow import InstalledAppFlow
        from googleapiclient.discovery import build
    except ImportError as exc:
        raise SystemExit(
            "Missing Google API packages. Install with:\n"
            "  python3 -m pip install -r requirements-youtube.txt"
        ) from exc

    credentials = None
    if token_path.exists():
        credentials = Credentials.from_authorized_user_file(str(token_path), SCOPES)

    if credentials and credentials.expired and credentials.refresh_token:
        credentials.refresh(Request())

    if not credentials or not credentials.valid:
        if not client_secrets.exists():
            raise SystemExit(f"Missing OAuth client secrets: {client_secrets}")
        flow = InstalledAppFlow.from_client_secrets_file(str(client_secrets), SCOPES)
        credentials = flow.run_local_server(port=0)
        token_path.parent.mkdir(parents=True, exist_ok=True)
        token_path.write_text(credentials.to_json())

    return build("youtube", "v3", credentials=credentials)


def media_upload(path: Path, mimetype: str | None = None):
    from googleapiclient.http import MediaFileUpload

    return MediaFileUpload(str(path), mimetype=mimetype, chunksize=-1, resumable=True)


def request_execute_with_progress(request, label: str) -> dict[str, Any]:
    response = None
    while response is None:
        status, response = request.next_chunk()
        if status:
            print(f"{label}: {int(status.progress() * 100)}%")
    return response


def current_channel_title(youtube) -> str:
    response = youtube.channels().list(part="snippet", mine=True).execute()
    items = response.get("items", [])
    if not items:
        return "unknown channel"
    snippet = items[0].get("snippet", {})
    return f"{snippet.get('title', 'unknown')} ({items[0].get('id', 'unknown id')})"


def find_playlist(youtube, title: str) -> str | None:
    page_token = None
    while True:
        response = youtube.playlists().list(
            part="snippet",
            mine=True,
            maxResults=50,
            pageToken=page_token,
        ).execute()
        for item in response.get("items", []):
            if item.get("snippet", {}).get("title") == title:
                return item.get("id")
        page_token = response.get("nextPageToken")
        if not page_token:
            return None


def ensure_playlist(youtube, title: str) -> str:
    existing = find_playlist(youtube, title)
    if existing:
        return existing
    response = youtube.playlists().insert(
        part="snippet,status",
        body={
            "snippet": {
                "title": title,
                "description": (
                    "Short visual clips for Bio Memory Rust Lab lessons on Rust, "
                    "molecules, data structures, and scientific feedback loops."
                ),
            },
            "status": {"privacyStatus": "unlisted"},
        },
    ).execute()
    return response["id"]


def upload_video(youtube, row: dict[str, str], video_path: Path, category_id: str, privacy: str) -> str:
    body = {
        "snippet": {
            "title": row["title"],
            "description": row["description"],
            "tags": split_tags(row.get("tags", "")),
            "categoryId": category_id,
        },
        "status": {
            "privacyStatus": privacy,
            "selfDeclaredMadeForKids": False,
        },
    }
    request = youtube.videos().insert(
        part="snippet,status",
        body=body,
        media_body=media_upload(video_path, "video/mp4"),
    )
    response = request_execute_with_progress(request, row["title"])
    return response["id"]


def set_thumbnail(youtube, video_id: str, thumbnail_path: Path) -> None:
    youtube.thumbnails().set(
        videoId=video_id,
        media_body=media_upload(thumbnail_path, "image/png"),
    ).execute()


def add_to_playlist(youtube, playlist_id: str, video_id: str) -> None:
    youtube.playlistItems().insert(
        part="snippet",
        body={
            "snippet": {
                "playlistId": playlist_id,
                "resourceId": {
                    "kind": "youtube#video",
                    "videoId": video_id,
                },
            }
        },
    ).execute()


def existing_links(path: Path) -> dict[str, str]:
    if not path.exists():
        return {}
    links = {}
    for line in path.read_text().splitlines():
        if not line.startswith("|") or line.startswith("| ---") or "YouTube URL" in line:
            continue
        parts = [part.strip() for part in line.strip("|").split("|")]
        if len(parts) >= 2:
            links[parts[0]] = parts[1]
    return links


def write_links_table(
    path: Path,
    all_rows: list[dict[str, str]],
    results: list[dict[str, str]],
) -> None:
    links = existing_links(path)
    links.update({result["title"]: result["url"] for result in results})
    lines = [
        "# YouTube Links",
        "",
        "Uploaded URLs for the Bio Memory Rust Lab visual clips.",
        "",
        "| Clip | YouTube URL |",
        "| --- | --- |",
    ]
    for row in all_rows:
        title = row["title"]
        lines.append(f"| {title} | {links.get(title, '')} |")
    path.write_text("\n".join(lines) + "\n")


def dry_run(rows: list[dict[str, str]], metadata_path: Path, allow_public: bool) -> None:
    print("Dry run only. Add --execute to upload.")
    for row in rows:
        privacy = normalize_privacy(row["visibility"], allow_public)
        video_path = resolve_repo_path(metadata_path, row["video_file"])
        thumb_path = resolve_repo_path(metadata_path, row["thumbnail_file"])
        print(f"- {row['title']}")
        print(f"  video: {video_path}")
        print(f"  thumbnail: {thumb_path}")
        print(f"  visibility: {privacy}")
        print(f"  playlist: {row['playlist']}")


def main() -> None:
    args = parse_args()
    all_rows = read_rows(args.metadata)
    upload_rows = all_rows[: args.limit] if args.limit is not None else all_rows
    if not upload_rows:
        raise SystemExit(f"No rows found in {args.metadata}")

    if not args.execute:
        dry_run(upload_rows, args.metadata, args.allow_public)
        return

    youtube = load_youtube(args.client_secrets, args.token)
    print(f"Authenticated channel: {current_channel_title(youtube)}")

    playlist_title = args.playlist or upload_rows[0]["playlist"]
    playlist_id = None if args.skip_playlist else ensure_playlist(youtube, playlist_title)
    results = []

    for row in upload_rows:
        privacy = normalize_privacy(row["visibility"], args.allow_public)
        video_path = resolve_repo_path(args.metadata, row["video_file"])
        thumbnail_path = resolve_repo_path(args.metadata, row["thumbnail_file"])
        if not video_path.exists():
            raise SystemExit(f"Missing video: {video_path}")
        if not args.skip_thumbnails and not thumbnail_path.exists():
            raise SystemExit(f"Missing thumbnail: {thumbnail_path}")

        print(f"Uploading: {row['title']}")
        video_id = upload_video(youtube, row, video_path, args.category_id, privacy)
        if not args.skip_thumbnails:
            set_thumbnail(youtube, video_id, thumbnail_path)
        if playlist_id:
            add_to_playlist(youtube, playlist_id, video_id)

        url = f"https://www.youtube.com/watch?v={video_id}"
        results.append({"title": row["title"], "video_id": video_id, "url": url})
        print(f"Uploaded: {url}")

    args.results.write_text(json.dumps(results, indent=2) + "\n")
    write_links_table(args.links, all_rows, results)
    print(f"Wrote links: {args.links}")


if __name__ == "__main__":
    main()
