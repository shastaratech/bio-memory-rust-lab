# YouTube API Upload Setup

This guide automates upload to the ShastaraTech YouTube channel:

<https://www.youtube.com/@ShastaraTech>

The Google Cloud project is:

<https://console.cloud.google.com/welcome?project=shastara>

## One-Time Google Cloud Setup

1. Open the `shastara` project in Google Cloud Console.
2. Enable **YouTube Data API v3**.
3. Configure the OAuth consent screen.
4. Add your Google account as a test user if the app is still in testing.
5. Create an OAuth client ID for a **Desktop app**.
6. Download the OAuth JSON file.
7. Save it outside Git:

```bash
mkdir -p ~/.config/shastaratech/youtube
cp ~/Downloads/client_secret_*.json ~/.config/shastaratech/youtube/client_secret.json
chmod 600 ~/.config/shastaratech/youtube/client_secret.json
```

Do not commit the OAuth JSON file or generated token file.

## Install Python Dependencies

```bash
python3 -m venv .venv-youtube
source .venv-youtube/bin/activate
python -m pip install -r requirements-youtube.txt
```

## Dry Run

```bash
python tools/youtube_upload.py
```

The default mode prints the upload plan without contacting YouTube.

## Upload One Clip As Unlisted

```bash
python tools/youtube_upload.py --execute --limit 1
```

The browser will ask you to sign in and choose the YouTube channel. Choose the
account/channel that owns `@ShastaraTech`.

## Upload All Clips

```bash
python tools/youtube_upload.py --execute
```

The script reads `metadata.csv`, uploads each MP4 as `Unlisted`, sets the custom
thumbnail, creates or reuses the `Bio Memory Rust Lab Visual Clips` playlist, and
writes final URLs into `youtube-links.md`.

## Safety Defaults

- Uploads are `Unlisted` from `metadata.csv`.
- Public upload is blocked unless you pass `--allow-public`.
- OAuth files are stored under `~/.config/shastaratech/youtube/`.
- `upload-results.local.json` is ignored by Git.

## Official References

- YouTube upload guide:
  <https://developers.google.com/youtube/v3/guides/uploading_a_video>
- OAuth guide:
  <https://developers.google.com/youtube/v3/guides/authentication>
- Quota calculator:
  <https://developers.google.com/youtube/v3/determine_quota_cost>
- Thumbnail API:
  <https://developers.google.com/youtube/v3/docs/thumbnails/set>
- Playlist API:
  <https://developers.google.com/youtube/v3/docs/playlists/insert>
