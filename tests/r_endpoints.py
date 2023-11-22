import requests
# i dont want to use pytest


def test_root():
    r = requests.get('http://127.0.0.1:8080/')
    print(r.status_code)
    print(r.content.decode())

def test_hey():
    r = requests.get('http://127.0.0.1:8080/hey')
    print(r.status_code)
    print(r.content.decode())

if __name__ == '__main__':
    test_root()
    test_hey()