import requests

r = requests.post("http://18.130.245.224/game/create_trap", json={"id": "4229965d-6cd1-411f-ba8f-33cdf0522c21", "state": "ready", "trap": "electricity", "color": "#000000", "text": "hi"})
print(r.url)
print(r.content)
print(r.status_code)
