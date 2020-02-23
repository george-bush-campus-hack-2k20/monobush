import requests
import heartbeat


r = requests.post("http://127.0.0.1:3000/request_trap", json={"id": "ee47ca46-d606-468b-a1b5-7d8ccb1d12ad"})
print(r.url)
print(r.content)
print(r.status_code)
