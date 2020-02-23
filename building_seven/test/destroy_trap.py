import requests

r = requests.post("http://127.0.0.1:3000/game/destroy_trap", json={"id": "4229965d-6cd1-411f-ba8f-33cdf0522c21"})
print(r.url)
print(r.content)
print(r.status_code)
