#!/usr/bin/python
# pip install youtube_transcript_api
from youtube_transcript_api import YouTubeTranscriptApi
import youtube_dl
import textwrap
import json

ydl = youtube_dl.YoutubeDL({'outtmpl': '%(id)s.%(ext)s'})
output_dict = []

with ydl:
    result = ydl.extract_info(
        'https://www.youtube.com/playlist?list=PLJwv6sN_mnF0QsOTcKlFDeyzwXMM0MWru',
        download=False  # We just want to extract the info
    )

    for video in result["entries"]:
        print("\n" + "*" * 60 + "\n")

        if not video:
            print("ERROR: Unable to get info. Continuing...")
            continue

        for prop in ["thumbnail", "id", "title", "description", "duration"]:
            print(prop + "\n" +
                  textwrap.indent(str(video.get(prop)),
                                  "    | ", lambda _: True)
                  )

    for video in result['entries']:
        if not video:
            print("ERROR: Unable to get info. Continuing...")
            continue

        try:
            t = YouTubeTranscriptApi.get_transcript(video.get("id"))
            output_dict.append(
                {
                    "id": video.get("id"),
                    "thumbnail": video.get("thumbnail"),
                    "title": video.get("title"),
                    "transcription": t
                }
            )

        except Exception as e:
            print("Could not read video: ", id)


print(output_dict)
with open("data/transcripts.json", "w") as f:
    f.write(json.dumps(output_dict, indent=4))
