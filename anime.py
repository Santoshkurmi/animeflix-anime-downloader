import requests


URL = "https://api.animeflix.live"
search= "/info/?query=data&limit=15"

data = requests.get(URL+"/info/?query=one piece&limit=15").json();

for i in range(len(data)):
    print(f"\n\n\n\n{data[i]}");



#EXTM3U
#EXT-X-STREAM-INF:PROGRAM-ID=1,BANDWIDTH=125214,RESOLUTION=640x360,NAME="360p"
ep.1.1677593055.360.m3u8
#EXT-X-STREAM-INF:PROGRAM-ID=1,BANDWIDTH=202097,RESOLUTION=854x480,NAME="480p"
ep.1.1677593055.480.m3u8
#EXT-X-STREAM-INF:PROGRAM-ID=1,BANDWIDTH=430177,RESOLUTION=1280x720,NAME="720p"
ep.1.1677593055.720.m3u8
#EXT-X-STREAM-INF:PROGRAM-ID=1,BANDWIDTH=918206,RESOLUTION=1920x1080,NAME="1080p"
ep.1.1677593055.1080.m3u8