import requests
import time
import webbrowser

api_url = input()
print('\a')
if (api_url == ""):
    api_url = "https://lizard.spacestation14.io/server/status"
response = requests.get(api_url)
print(response.status_code)
json = response.json()

while (json["players"] >= 75):
    time.sleep(15)
    response = requests.get(api_url)
    json = response.json()
    print(json["players"])

webbrowser.open('https://www.google.com/search?client=firefox-b-1-d&q=join+the+server+now')
