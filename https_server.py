import http.server
import ssl
import os

PORT = 8000
CERTFILE = "/home/flappy-bird/cert.pem"
KEYFILE = "/home/flappy-bird/key.pem"
DIRECTORY = "/home/SuccinverseBird-JS"

class MyHandler(http.server.SimpleHTTPRequestHandler):
    def translate_path(self, path):
        """Dosya yolunu güvenli bir şekilde oluştur."""
        root = os.path.abspath(DIRECTORY)
        child = os.path.normpath(os.path.join(root, path.lstrip('/')))
        if not child.startswith(root):
            return None  # Güvenlik nedeniyle root dizininin dışına çıkmaya izin verme
        return child

os.chdir(DIRECTORY)  # Doğru dizinde çalıştığımızdan emin ol

httpd = http.server.HTTPServer(("", PORT), MyHandler)

try:
    httpd.socket = ssl.wrap_socket(httpd.socket, certfile=CERTFILE, keyfile=KEYFILE, server_side=True)
    print(f"HTTPS sunucusu https://nizamulmulk.xyz:{PORT} adresinde başlatıldı.")
    httpd.serve_forever()
except FileNotFoundError:
    print("Sertifika veya anahtar dosyaları bulunamadı.")
except Exception as e:
    print(f"HTTPS sunucusu başlatılırken bir hata oluştu: {e}")